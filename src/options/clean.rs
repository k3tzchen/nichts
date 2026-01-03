use crate::{ Operation, command::exec_cmd, error::Error };

pub struct Clean;

impl Operation for Clean {
  fn operate(_cli: &crate::Cli) -> Result<(), Error> {
    return exec_cmd("nix-collect-garbage --verbose", false);
  }
}
