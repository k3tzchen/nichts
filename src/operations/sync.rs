use crate::{Operation, command::execute_command, error::Error, operations::Operations, options::{Options, clean::Clean, search::Search, upgrade::Upgrade} };

pub struct Sync;

impl Operation for Sync {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Sync);
      return Ok(());
    }

    let packages = &cli.packages;

    if cli.update {
      return Upgrade::operate(&cli);
    }

    if cli.search {
      if cli.packages.is_empty() {
        return Err(Error::NotSpecified { kind: "pattern(s)".to_string() });
      }

      return Search::operate(&cli);
    }

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

    let mut command = format!("nix profile add {profile} {log_level}");

    let package_prefix =
      if cli.flake.is_some() { format!("{}#", cli.flake.clone().unwrap()) }
      else { "nixpkgs#".to_string() };

    if cli.impure {
      command.push_str(" --impure");
    }

    if cli.refresh {
      command.push_str(" --refresh");
    }

    for package in packages  {
      execute_command(format!("{command} -- {package_prefix}{package}"), false)?;
    }

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
