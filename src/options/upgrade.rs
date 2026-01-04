use crate::{Operation, command::execute_command, error::Error, options::clean::Clean};

pub struct Upgrade;

impl Operation for Upgrade {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    let packages = cli.packages.join(" ");

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

    let mut command = format!("nix profile upgrade {profile} {log_level}");

    if cli.impure {
      command.push_str(" --impure");
    }

    if cli.refresh {
      command.push_str(" --refresh");
    }

    if packages.is_empty() {
      return execute_command(format!("{command} --all"), false);
    }

    execute_command(format!("{command} -- {packages}"), false)?;

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
