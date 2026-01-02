use super::{Operation, Operations};
use crate::{ command::exec_cmd, options::{Options, clean::Clean} };

pub struct Remove;

impl Operation for Remove {
  fn operate(cli: &crate::Cli) -> Result<(), (i32, &str)> {
    if cli.help {
      Options::print_help(Operations::Remove);
      return Ok(());
    }

    if cli.packages.is_empty() {
      return Err((1, "no target(s) specified"));
    }

    let packages = &cli.packages;
    if let Err(err) = exec_cmd(format!("nix profile remove -- {}", packages.join(" ")), false) {
      return Err(err);
    }

    if cli.clean {
      return Clean::operate(&cli);
    }

    return Ok(());
  }
}
