extern crate clap;
extern crate regex;
extern crate skim;

use regex::Regex;
use skim::prelude::*;
use std::io::{Cursor, Read};
use std::process::Command;

mod cli;
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

pub fn main() {
    let config = cli::config();

    match &config.subcommand {
        cli::Subcommand::Jump => branch_hop(&config),
        cli::Subcommand::Tutorial => tutorial::print_tutorial(),
    }
}

pub fn branch_hop(config: &cli::Config) {
    ensure_clean_local_repo(&config);

    let current_branch_name = current_branch_name();
    let target_branch = pick_target_branch(&current_branch_name, &config);
    let branch_point = pick_branch_point(&current_branch_name, &target_branch, &config);

    println!("The Rebase Wizard has seen your future:");
    println!("");
    println!("Run the following command when you are ready:");
    println!("  git rebase --onto {} {}", target_branch, branch_point);
}

pub fn pick_target_branch(current_branch_name: &str, config: &cli::Config) -> String {
    let header_str = format!(
        "Pick the TARGET_BRANCH. This will be the new base branch for {} after we finish this jump.",
        current_branch_name
    );
    let preview_window = config.preview_window_location.as_arg();
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

pub fn pick_branch_point(
    current_branch_name: &str,
    target_branch: &str,
    config: &cli::Config,
) -> String {
    let header_str = format!(
        "Pick the BRANCH_POINT. This will be the first commit you didn't author on {}.",
        current_branch_name
    );
    let preview_window = config.preview_window_location.as_arg();
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

static LOCAL_CHANGES_WARNING: &'static str = "\
The Wizard advises against rebasing while there are changes to the local git repo.

Please commit, stash, or discard your changes before proceeding.";
fn ensure_clean_local_repo(config: &cli::Config) {
    if !local_repo_is_clean() {
        eprintln!("{}", &LOCAL_CHANGES_WARNING);
        if !config.dev_mode {
            std::process::exit(1);
        }
    }
}

fn local_repo_is_clean() -> bool {
    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .output()
        .expect("Failed to determine local repo status")
        .stdout;

    output.is_empty()
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
