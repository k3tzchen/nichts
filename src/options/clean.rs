use crate::{ Operation, command::exec_cmd };

pub struct Clean;

impl Operation for Clean {
  fn operate(_cli: &crate::Cli) -> Result<(), (i32, &str)> {
    return exec_cmd("nix-collect-garbage --verbose", false);
  }
}
