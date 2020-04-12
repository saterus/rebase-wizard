![Rust Build](https://github.com/saterus/rebase-wizard/workflows/Rust%20Build/badge.svg)

# Rebase Wizard

The Wizard can help with all your rebase problems!

## IMPORTANT NOTE:

The Wizard never acts without your intervention. The Wizard only provides advice and **never** runs a command that could change your git history **in any way**. That's your job.

## Usage

### See how the wizard can help!
```bash
$ rebase-wizard --help
$ rebase-wizard --tutorial
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
$ rebase-wizard help
$ rebase-wizard help jump
$ rebase-wizard jump
```

## User Setup:

### Requirements:
* git 2.23+

##### OSX:

Please bear us as we improve these cumbersome distribution problems. We're actively trying to improve it.

###### Homebrew Installation
```bash
# Add the custom homebrew "tap" using ssh protocol
$ brew tap saterus/rebase-wizard git@github.com:saterus/rebase-wizard.git
# OR
# Add the custom homebrew "tap" using https protocol
$ brew tap saterus/rebase-wizard https://github.com/saterus/rebase-wizard

# Install the latest published version
$ brew install rebase-wizard
```

###### Manual Installation:
* Download the latest build from the [Releases page](https://github.com/saterus/rebase-wizard/releases).
* `chmod 755 rebase-wizard` so it is executable.
* Add the `rebase-wizard` to your `/usr/local/bin` so it is available in your `$PATH`.
* Be ready to open the Security Settings panel and allow an exception for this unsigned binary to run. :grimacing:

##### Linux/Windows:
* Untested
* Follow the Developer Setup steps

## Developer Setup:

### Requirements:
* git 2.23+
* Rust environment

#### Setup Rust Dev Environment:

You can skip this step if you already have Rust installed.

```bash
# Install the Rust toolchain manager: rustup
$ brew install rustup-init
$ rustup-init

# Add the Cargo binary directory to your path
$ source ~/.cargo/env

# Verify you have a working Rust installation.
$ rustup show

# =>
# Should output something similar to:
# Default host: x86_64-apple-darwin
# rustup home:  ~/.rustup

# installed toolchains
# --------------------
# stable-x86_64-apple-darwin


# Clone this repo
$ git clone git@github.com:saterus/rebase-wizard.git

# Build a local copy
$ cargo build
```

#### Install the binary in your path
```bash
# Install into ~/.cargo/bin
$ cargo install --path .

# Run from any git repository
$ rebase-wizard
```

# Contributing

**Please provide feedback.** This is a super rough prototype. Barely more than
a distributable shell script. I'd love to make it better.
