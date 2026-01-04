use super::{Operation, Operations};
use crate::{Cli, command::execute_command, error::Error, options::{Options, clean::Clean}};

pub struct History;

impl Operation for History {
  fn operate(cli: &Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::History);
      return Ok(());
    }

    if cli.packages.is_empty() {
      if cli.clean {
        return Err(Error::NotSpecified { kind: "generation".to_string() });
      }

      return execute_command("nix profile history", false);
    }

    if let Some(rollback) = cli.packages.get(0) {
      let profile = cli.profile.clone().map(|profile| {
        if !profile.is_empty() {
          return format!("--profile {profile}");
        }

        return profile;
      }).unwrap_or_else(|| "".to_string());

      execute_command(format!("nix profile rollback {profile} --to {rollback}"), false)?;

      if cli.clean {
        Clean::operate(&cli)?;
      }

      return Ok(());
    }

    Err(Error::FailedRollback)
  }
}
