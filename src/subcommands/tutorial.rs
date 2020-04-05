pub fn print_tutorial() {
    println!("{}", SECRETS_TEXT);
    println!("{}", TUTORIAL_TEXT);
}

pub static SECRETS_TEXT: &'static str = r"
Wizard Secrets: 🧙‍♂️

  git rebase --onto TARGET_BRANCH  BRANCH_POINT
                          ^              ^
                          |              |
                          |              |
                          |              |
                          +              |
        Latest mainline branch           |
                                         +
                                 First commit you didn't author on this branch
";

pub static MORE_HELP_TEASER: &'static str = "For more secrets 🔮, try running the --tutorial flag.";

pub static TUTORIAL_TEXT: &'static str = r"
  *Branch Jumping*: It isn't uncommon to need to move your feature branch from
  being based on one branch to another. This requires picking the exact right
  arguments for `git rebase --onto`.

  To illustrate how this needs to work, let us take the following example:

    I---o---o---o---J  master
    \
      o---o---o---o---X  train/2020-04-01
                      \
                        A---B---C  feature-omega

  Let's say you miss the train and a new one is created:

                        o---o---Y train/2020-04-02
                       /
  I---o---o---o---J---K  master
   \                 /
    o---o---o---o---X  train/2020-04-01
                      \
                        A---B---C  feature-omega

  Fear not! We can hop aboard that train instead.

    git rebase --onto train/2020-04-02 train/2020-04-01
    aka:
    git rebase --onto commit-Y         commit-X

  This will replay all our commits from C back to A on top of Y. This
  creates new commits with the same contents on top of the new base branch.

                                  A'--B'--C' feature-omega
                                 /
                        o---o---Y train/2020-04-02
                       /
  I---o---o---o---J---K  master
   \                 /
    o---o---o---o---X  train/2020-04-01

  Now the feature-omega branch is updated and riding aboard the next train.

  🧙‍♂  ➡️  🚂  🎉
 ";
