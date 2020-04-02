# Rebase Wizard

The Wizard can help with all your rebase problems!

**IMPORTANT NOTE**: The Wizard will never act without your permission.
Currently the Wizard only provides advice and **never** runs a command that
could change your git history **in any way**. That's your job.

### (Developer) Setup:

Pre-Requirements:
* Rust 1.40+
* cargo
* `~/.cargo/bin` added to your path

```bash
# Clone this repo
$ git clone git@github.com:saterus/rebase-wizard.git

# Install the binary
$ cargo install --path .

# See how the wizard can help!
$ rebase-wizard --help
```

### Base Branch Switching

Say you want to switch base branches for your feature branch. The Wizard can work some magic!

```bash
# Open your project
$ cd PROJECT_DIR

# Switch to your feature branch
$ git checkout best-feature-ever

# Make sure you've sync'd with your remotes.
$ git fetch

# Ask the wizard
$ rebase-wizard
```

### Other Rebase Operations

Not yet supported

# Contributing

**Please provide feedback.** This is a super rough prototype. Barely more than
a distributable shell script. I'd love to make it better.
