use crate::{ Operation, command::execute_command, error::Error };

pub struct Clean;

impl Operation for Clean {
  fn operate(_cli: &crate::Cli) -> Result<(), Error> {
    execute_command("nix-collect-garbage --verbose", false)
  }
}
