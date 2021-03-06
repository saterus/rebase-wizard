use crate::cli;
use crate::git;
use crate::sk;

pub fn jump(config: &cli::Config) {
    git::ensure_clean_local_repo(&config);

    let current_branch_name = git::current_branch_name();
    let target_branch = pick_target_branch(&current_branch_name, &config);
    let branch_point = pick_branch_point(&current_branch_name, &target_branch, &config);

    println!("The Rebase Wizard has seen your future:");
    println!();
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

    let options = sk::SimpleOptions {
        header: &header_str,
        prompt: "Select TARGET BRANCH: ",
        preview: preview_str,
        preview_window: &preview_window,
    };

    let branches = git::all_branches();
    let selection = sk::one(sk::to_source(branches), options);

    git::extract_ref(&selection)
        .expect("branch name not found")
        .to_string()
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

    let options = sk::SimpleOptions {
        header: &header_str,
        prompt: "Select BRANCH POINT: ",
        preview: &preview_str,
        preview_window: &preview_window,
    };

    let commits = git::recent_commits();
    let selection = sk::one(sk::to_source(commits), options);

    git::extract_ref(&selection)
        .expect("commit sha not found")
        .to_string()
}
