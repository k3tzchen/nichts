use crate::{CLI_NAME, CLI_VERSION, Operation, command::catch_output, error::Error, operations::Operations, options::Options};

pub struct Version;

impl Operation for Version {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Version);
      return Ok(());
    }

    let output = catch_output("nix --version", true)?;
    print!("{CLI_NAME} {CLI_VERSION} - {output}");

    Ok(())
  }
}
