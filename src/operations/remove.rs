use super::{Operation, Operations};
use crate::{ command::execute_command, error::Error, options::{Options, clean::Clean} };

pub struct Remove;

impl Operation for Remove {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Remove);
      return Ok(());
    }

    if cli.packages.is_empty() {
      return Err(Error::NotSpecified { kind: "target(s)".to_string() });
    }

    let packages = &cli.packages;

    let log_level = if cli.quiet {
      "--quiet"
    } else {
      "--verbose"
    };

    let profile = cli.profile.clone().map(|profile| {
      if !profile.is_empty() {
        return format!("--profile {profile}");
      }

      return profile;
    }).unwrap_or_else(|| "".to_string());

    execute_command(format!("nix profile remove {profile} {log_level} -- {}", packages.join(" ")), false)?;

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
