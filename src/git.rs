use crate::cli;
use regex::Regex;
use std::io::{BufRead, Cursor, Read};
use std::process::Command;

pub enum BranchLocation {
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

static LOCAL_CHANGES_WARNING: &'static str = "\
The Wizard advises against rebasing while there are changes to the local git repo.

Please commit, stash, or discard your changes before proceeding.";
pub fn ensure_clean_local_repo(config: &cli::Config) {
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

pub fn all_branches() -> impl BufRead {
    let local_branches = branch_list(BranchLocation::Local);
    let remote_branches = branch_list(BranchLocation::Remote);

    local_branches.chain(remote_branches)
}

pub fn branch_list(location: BranchLocation) -> impl BufRead {
    let branches = Command::new("git")
        .arg("branch")
        .arg("-vvv")
        .arg("--sort=-committerdate")
        .args(location.as_arg())
        .output()
        .expect("Failed to generate local branches")
        .stdout;

    Cursor::new(branches)
}

pub fn recent_commits() -> impl BufRead {
    let commits = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%h %ad | %s%d [%an]")
        .arg("--graph")
        .arg("--date=short")
        .arg("-n50")
        .output()
        .expect("Failed to generate recent commits")
        .stdout;

    Cursor::new(commits)
}

pub fn current_branch_name() -> String {
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

pub fn extract_ref(branch_line: &str) -> &str {
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
    fn test_extract_ref() {
        let sample = " * foo/bar-baz         770b0814b WIP!";
        let sample2 = "   foo/bark-bazz         770b0814b WIP!";
        let sample3 = "   origin/foo/bar-baz         770b0814b WIP!";
        let sample4 = "   fa1afe1         WIP!";

        assert_eq!(extract_ref(sample), "foo/bar-baz");
        assert_eq!(extract_ref(sample2), "foo/bark-bazz");
        assert_eq!(extract_ref(sample3), "origin/foo/bar-baz");
        assert_eq!(extract_ref(sample4), "fa1afe1");
    }
}
