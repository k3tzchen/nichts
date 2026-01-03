use super::{Operation, Operations};
use crate::{ command::exec_cmd, error::Error, options::{Options, clean::Clean} };

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

    if let Err(err) = exec_cmd(format!("nix profile remove {log_level} -- {}", packages.join(" ")), false) {
      return Err(err);
    }

    if cli.clean {
      return Clean::operate(&cli);
    }

    return Ok(());
  }
}
