use crate::tutorial;
use clap::arg_enum;

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
    #[structopt(subcommand)]
    pub subcommand: Subcommand,

    /// Set which side of the screen the preview window appears
    #[structopt(short, long, possible_values = &PreviewWindowLocation::variants(), case_insensitive = true, default_value = "Up")]
    pub preview_window_location: PreviewWindowLocation,

    /// Enable dev_mode mode. Only useful for developing rebase-wizard itself.
    #[structopt(short, long)]
    pub dev_mode: bool,
}

pub fn config() -> Opt {
    Opt::from_args()
}
