use super::{Operation, Operations};
use crate::{Cli, command::execute_command, error::Error, options::{Options, clean::Clean}};

pub struct History;

impl Operation for History {
  fn operate(cli: &Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::History);
      return Ok(());
    }

    let profile = cli.profile.clone().map(|profile| {
      if !profile.is_empty() {
        return format!("--profile {profile}");
      }

      return profile;
    }).unwrap_or_else(|| "".to_string());

    if cli.wipe.is_some() {
      let wipe_time = cli.wipe.clone().unwrap();

      let mut older_than = String::new();
      if !wipe_time.eq(&"0d") {
        older_than = format!("--older-than {wipe_time}");
      }

      execute_command(format!("nix profile wipe-history {profile} {older_than}"), false)?;

      return Ok(());
    }

    if cli.packages.is_empty() {
      if cli.clean {
        return Err(Error::NotSpecified { kind: "generation".to_string() });
      }

      execute_command("nix profile history", false)?;

      return Ok(());
    }

    if let Some(rollback) = cli.packages.get(0) {
      execute_command(format!("nix profile rollback {profile} --to {rollback}"), false)?;

      if cli.clean {
        Clean::operate(&cli)?;
      }

      return Ok(());
    }

    Err(Error::FailedRollback)
  }
}
