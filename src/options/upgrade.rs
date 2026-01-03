use crate::{Operation, command::exec_cmd, error::Error, operations::query::list_packages, options::clean::Clean};

pub struct Upgrade;

impl Operation for Upgrade {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    let mut packages = cli.packages.join(" ");

    let log_level = if cli.quiet {
      "--quiet"
    } else {
      "--verbose"
    };

    let mut command = format!("nix profile upgrade {log_level}");

    if cli.impure {
      command.push_str(" --impure");
    }

    if cli.refresh {
      command.push_str(" --refresh");
    }

    if packages.is_empty() {
      let installed_packages = list_packages(false);
      if installed_packages.is_empty() {
        return Err(Error::NoPackageFound);
      }
      packages = installed_packages.join(" ");
    }

    if let Err(err) = exec_cmd(format!("{command} -- {packages}"), false) {
      return Err(err);
    }

    if cli.clean {
      return Clean::operate(&cli);
    }

    return Ok(());
  }
}
