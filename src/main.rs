extern crate clap;
extern crate regex;
extern crate skim;

mod cli;
mod git;
mod sk;
mod subcommands;

pub fn main() {
    let config = cli::config();

    match &config.subcommand {
        cli::Subcommand::Jump => subcommands::jump(&config),
    }
}
