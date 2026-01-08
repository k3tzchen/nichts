use crate::{Operation, command::execute_command, error::Error, options::clean::Clean};

pub struct Upgrade;

impl Operation for Upgrade {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    let packages = cli.packages.join(" ");
    let command = cli.prepare_command("nix profile upgrade");

    if packages.is_empty() {
      execute_command(format!("{command} --all"), false)?;
    } else {
      execute_command(format!("{command} -- {packages}"), false)?;
    }

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
