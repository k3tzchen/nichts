use super::{Operation, Operations};
use crate::{Cli, command::exec_cmd, options::{Options, clean::Clean}};

pub struct History;

impl Operation for History {
  fn operate(cli: &Cli) -> Result<(), (i32, &str)> {
    if cli.help {
      Options::print_help(Operations::History);
      return Ok(());
    }

    if cli.packages.is_empty() {
      if cli.clean {
        return Err((1, "no generation specified"));
      }

      return exec_cmd("nix profile history", false);
    }

    if let Some(rollback) = cli.packages.get(0) {
      if let Err(err) = exec_cmd(format!("nix profile rollback --to {rollback}"), false) {
        return Err(err);
      }

      if cli.clean {
        return Clean::operate(&cli);
      }

      return Ok(());
    }

    return Err((1, "Failed to roll back"));
  }
}
