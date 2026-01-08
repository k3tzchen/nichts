use std::process::exit;

use clap::Parser;
use clap::error::ErrorKind;

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

mod api;
mod operations;
mod options;
mod command;
mod error;

pub static CLI_NAME: &str = env!("CARGO_BIN_NAME");
pub static CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static DEFAULT_FLAKE: &str = "flake:nixpkgs";

#[derive(Parser, Debug)]
#[command(name = CLI_NAME)]
#[command(disable_help_flag = true)]
#[command(disable_version_flag = true)]
struct Cli {
  #[arg(short = Operations::Help.short(), long = Operations::Help.long(), action = clap::ArgAction::SetTrue)]
  help: bool,

  #[arg(short = Operations::History.short(), long = Operations::History.long(), action = clap::ArgAction::SetTrue)]
  history: bool,

  #[arg(short = Operations::Query.short(), long = Operations::Query.long(), action = clap::ArgAction::SetTrue)]
  query: bool,

  #[arg(short = Operations::Remove.short(), long = Operations::Remove.long(), action = clap::ArgAction::SetTrue)]
  remove: bool,

  #[arg(short = Operations::Sync.short(), long = Operations::Sync.long(), action = clap::ArgAction::SetTrue)]
  sync: bool,

  #[arg(short = Operations::Version.short(), long = Operations::Version.long(), action = clap::ArgAction::SetTrue)]
  version: bool,

  #[arg(short = Options::Clean.short(), long = Options::Clean.long(), action = clap::ArgAction::SetTrue)]
  clean: bool,

  #[arg(long = Options::Flake.long(), num_args = 0..=1, default_value = None, default_missing_value = Some(""))]
  flake: Option<String>,

  #[arg(long = Options::Impure.long(), action = clap::ArgAction::SetTrue)]
  impure: bool,

  #[arg(short = Options::Info.short(), long = Options::Info.long(), action = clap::ArgAction::SetTrue)]
  info: bool,

  #[arg(long = Options::Json.long(), action = clap::ArgAction::SetTrue)]
  json: bool,

  #[arg(long = Options::NoConfirm.long(), action = clap::ArgAction::SetTrue)]
  noconfirm: bool,

  #[arg(long = Options::Profile.long(), num_args = 0..=1, default_value = None, default_missing_value = Some(""))]
  profile: Option<String>,

  #[arg(short = Options::Quiet.short(), long = Options::Quiet.long(), action = clap::ArgAction::SetTrue)]
  quiet: bool,

  #[arg(short = Options::Refresh.short(), long = Options::Refresh.long(), action = clap::ArgAction::SetTrue)]
  refresh: bool,

  #[arg(long = Options::Rollback.long(), action = clap::ArgAction::SetTrue)]
  rollback: bool,

  #[arg(short = Options::Search.short(), long = Options::Search.long(), action = clap::ArgAction::SetTrue)]
  search: bool,

  #[arg(short = Options::Upgrade.short(), long = Options::Upgrade.long(), action = clap::ArgAction::SetTrue)]
  upgrade: bool,

  #[arg(long = Options::Wipe.long(), num_args = 0..=1, default_missing_value = Some(""))]
  wipe: Option<String>,

  packages: Vec<String>,
}

impl Cli {
  fn flake_url(&self) -> &str {
    if let Some(flake) = self.flake.as_deref() {
      return &flake;
    }

    return DEFAULT_FLAKE;
  }

  fn profile(&self) -> String {
    if let Some(profile) = self.profile.as_deref() {
      if !profile.is_empty() {
        return format!(" --profile {profile}");
      }
    }

    String::new()
  }

  fn prepare_command(&self, command: impl Into<String>) -> String {
    let mut command = command.into();

    if self.impure {
      command.push_str(" --impure");
    }

    if self.refresh {
      command.push_str(" --refresh");
    }

    if self.quiet {
      command.push_str(" --quiet");
    } else {
      command.push_str(" --verbose");
    }

    let profile = self.profile();
    if !profile.is_empty() {
      command.push_str(&profile);
    }

    command
  }
}

fn main() {
  let cli = match Cli::try_parse() {
    Ok(cli) => cli,
    Err(err) => {
      match err.kind() {
        ErrorKind::UnknownArgument => {
          if let Some((_, value)) = err.context().next() {
            Operations::throw_if_needed(Err(Error::UnknownOption {
              option: value.to_string(),
            }));
          }
        },
        _ => {
          eprintln!("{}", err);
        }
      }

      exit(err.exit_code());
    }
  };

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
