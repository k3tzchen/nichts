use clap::Parser;

use crate::error::Error;
use crate::operations::history::History;
use crate::operations::{ Operation, Operations };
use crate::operations::{
  query::Query,
  sync::Sync,
  version::Version,
  help::Help,
  remove::Remove
};

use crate::options::{ Options};
use crate::options::clean::Clean;

mod operations;
mod options;
mod command;
mod error;
mod package;

pub static CLI_NAME: &str = env!("CARGO_BIN_NAME");
pub static CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = CLI_NAME)]
#[command(disable_help_flag = true)]
#[command(disable_version_flag = true)]
struct Cli {
  #[arg(short = Operations::Sync.short(), long = Operations::Sync.long(), action = clap::ArgAction::SetTrue)]
  sync: bool,

  #[arg(short = Operations::Remove.short(), long = Operations::Remove.long(), action = clap::ArgAction::SetTrue)]
  remove: bool,

  #[arg(short = Operations::Query.short(), long = Operations::Query.long(), action = clap::ArgAction::SetTrue)]
  query: bool,

  #[arg(short = Operations::Version.short(), long = Operations::Version.long(), action = clap::ArgAction::SetTrue)]
  version: bool,

  #[arg(short = Operations::Help.short(), long = Operations::Help.long(), action = clap::ArgAction::SetTrue)]
  help: bool,

  #[arg(short = Operations::History.short(), long = Operations::History.long(), action = clap::ArgAction::SetTrue)]
  history: bool,

  #[arg(short = Options::Upgrade.short(), long = Options::Upgrade.long(), action = clap::ArgAction::SetTrue)]
  update: bool,

  #[arg(long = Options::Flake.long(), default_value = None)]
  flake: Option<String>,

  #[arg(long = Options::Profile.long(), default_value = None)]
  profile: Option<String>,

  #[arg(long = Options::Wipe.long(), num_args = 0..=1, default_missing_value = Some("0d"))]
  wipe: Option<String>,

  #[arg(short = Options::Search.short(), long = Options::Search.long(), action = clap::ArgAction::SetTrue)]
  search: bool,

  #[arg(long = Options::Impure.long(), action = clap::ArgAction::SetTrue)]
  impure: bool,

  #[arg(short = Options::Refresh.short(), long = Options::Refresh.long(), action = clap::ArgAction::SetTrue)]
  refresh: bool,

  #[arg(short = Options::Clean.short(), long = Options::Clean.long(), action = clap::ArgAction::SetTrue)]
  clean: bool,

  #[arg(short = Options::Info.short(), long = Options::Info.long(), action = clap::ArgAction::SetTrue)]
  info: bool,

  #[arg(short = Options::Quiet.short(), long = Options::Quiet.long(), action = clap::ArgAction::SetTrue)]
  quiet: bool,

  packages: Vec<String>,
}

fn main() {
  let cli = Cli::parse();

  let command_count = [cli.query, cli.remove, cli.sync, cli.version, cli.history]
    .iter()
    .filter(|&&x| x)
    .count();

  if command_count == 0 {
    if cli.help {
      Operations::throw_if_needed(Help::operate(&cli));
    }

    if cli.clean {
      Operations::throw_if_needed(Clean::operate(&cli));
    }

    Operations::throw_if_needed(Err(Error::NotSpecified { kind: "operation".to_string() }));
  }

  if command_count > 1 {
    Operations::throw_if_needed(Err(Error::Unknown { code: 1, message: "only one operation may be used at a time".to_string() }));
  }

  Operations::throw_if_needed(match () {
    _ if cli.sync => Sync::operate(&cli),
    _ if cli.query => Query::operate(&cli),
    _ if cli.version => Version::operate(&cli),
    _ if cli.remove => Remove::operate(&cli),
    _ if cli.history => History::operate(&cli),
    _ if cli.help => Help::operate(&cli),
    _ => Ok(())
  });
}
