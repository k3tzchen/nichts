use crate::{CLI_NAME, CLI_VERSION, Operation, command::prepare_cmd, error::Error, operations::Operations, options::Options};

pub struct Version;

impl Operation for Version {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Version);
      return Ok(());
    }

    let output = prepare_cmd(vec!["nix", "--version"], true)
      .output()
      .expect("Failed to get nix version");

    let output = String::from_utf8_lossy(&output.stdout);

    print!("{CLI_NAME} {CLI_VERSION} - {output}");

    return Ok(());
  }
}
