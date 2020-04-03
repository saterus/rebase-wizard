extern crate clap;
extern crate regex;
extern crate skim;

use clap::arg_enum;
use regex::Regex;
use skim::prelude::*;
use std::fmt::Debug;
use std::io::{Cursor, Read};
use std::process::Command;

mod tutorial;

enum BranchLocation {
    Local,
    Remote,
}

impl BranchLocation {
    pub fn as_arg(&self) -> Vec<String> {
        match self {
            BranchLocation::Local => vec!["--list".to_string()],
            BranchLocation::Remote => vec!["--list".to_string(), "--remote".to_string()],
        }
    }
}

arg_enum! {
  #[derive(Debug)]
  pub enum PreviewWindowLocation {
    Up,
    Down,
    Left,
    Right,
  }
}

impl PreviewWindowLocation {
    pub fn as_arg(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Jump between base branches.
    #[structopt(name = "jump")]
    Jump,
    /// View the rebase-wizard tutorial about branch jumping.
    Tutorial,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rebase_wizard", setting = AppSettings::InferSubcommands, after_help = tutorial::MORE_HELP_TEASER, long_about = tutorial::SECRETS_TEXT)]
pub struct Opt {
    #[structopt(short, long, possible_values = &PreviewWindowLocation::variants(), case_insensitive = true, default_value = "Up")]
    preview_window_location: PreviewWindowLocation,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

pub fn main() {
    let opt = Opt::from_args();

    match &opt.subcommand {
        Subcommand::Jump => branch_hop(&opt),
        Subcommand::Tutorial => tutorial::print_tutorial(),
    }
}

pub fn branch_hop(opt: &Opt) {
    let current_branch_name = current_branch_name();
    let target_branch = pick_target_branch(&current_branch_name, &opt);
    let branch_point = pick_branch_point(&current_branch_name, &target_branch, &opt);

    println!("The Rebase Wizard has seen your future:");
    println!("");
    println!("Run the following command when you are ready:");
    println!("  git rebase --onto {} {}", target_branch, branch_point);
}

pub fn pick_target_branch(current_branch_name: &str, opt: &Opt) -> String {
    let header_str = format!(
        "Pick the TARGET_BRANCH. This will be the new base branch for {} after we finish this jump.",
        current_branch_name
    );
    let preview_window = opt.preview_window_location.as_arg();
    let preview_str = "\
      echo -e 'Rebase Command Preview:
        git rebase --onto {2}  BRANCH_POINT\n' &&\
      echo -e 'Branch HEAD Preview: (git show --stat {2})\n' &&\
        git show --stat {2}";

    let options = SkimOptions {
        header: Some(&header_str),
        prompt: Some("Select TARGET BRANCH: "),
        preview: Some(preview_str),
        preview_window: Some(&preview_window),
        ..SkimOptions::default()
    };

    let skim_output = Skim::run_with(&options, Some(all_branches()))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected_item = skim_output.first().unwrap_or_else(|| {
        eprintln!("Nothing selected. Aborting.");
        std::process::exit(1);
    });

    find_branch_name(&selected_item.output()).to_string()
}

pub fn pick_branch_point(current_branch_name: &str, target_branch: &str, opt: &Opt) -> String {
    let header_str = format!(
        "Pick the BRANCH_POINT. This will be the first commit you didn't author on {}.",
        current_branch_name
    );
    let preview_window = opt.preview_window_location.as_arg();
    let preview_str = format!(
        "\
      echo -e 'Rebase Command Preview:
        git rebase --onto {}  {{2}}\n' &&\
      echo -e 'Commit Preview: (git show --stat {{2}})\n' &&\
        git show --stat {{2}}",
        target_branch
    );

    let options = SkimOptions {
        cmd: Some("git log --pretty=format:\"%h %ad | %s%d [%an]\" --graph --date=short -n 50"),
        header: Some(&header_str),
        prompt: Some("Select BRANCH POINT: "),
        preview: Some(&preview_str),
        preview_window: Some(&preview_window),
        ..SkimOptions::default()
    };

    let skim_output = Skim::run_with(&options, None)
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected_item = skim_output.first().unwrap_or_else(|| {
        eprintln!("Nothing selected. Aborting.");
        std::process::exit(1);
    });

    find_branch_name(&selected_item.output()).to_string()
}

fn all_branches() -> SkimItemReceiver {
    let local_branch_output = branch_list(BranchLocation::Local);
    let remote_branch_output = branch_list(BranchLocation::Remote);

    let local_branches = Cursor::new(local_branch_output);
    let remote_branches = Cursor::new(remote_branch_output);

    let item_reader = SkimItemReader::default();
    item_reader.of_bufread(local_branches.chain(remote_branches))
}

fn branch_list(location: BranchLocation) -> Vec<u8> {
    Command::new("git")
        .arg("branch")
        .arg("-vvv")
        .arg("--sort=-committerdate")
        .args(location.as_arg())
        .output()
        .expect("Failed to generate local branches")
        .stdout
}

fn current_branch_name() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("Failed to determine current branch")
        .stdout;

    std::str::from_utf8(&output[..])
        .expect("Found non-unicode in current branch name?")
        .to_string()
}

pub fn find_branch_name(branch_line: &str) -> &str {
    let re = Regex::new(r"\s*(\w\S*)\s").unwrap();

    for caps in re.captures_iter(branch_line) {
        return caps.get(1).unwrap().as_str();
    }

    return "";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_branch_name() {
        let sample = " * foo/bar-baz         770b0814b WIP!";
        let sample2 = "   foo/bark-bazz         770b0814b WIP!";

        assert_eq!(find_branch_name(sample), "foo/bar-baz");
        assert_eq!(find_branch_name(sample2), "foo/bark-bazz");
    }
}
