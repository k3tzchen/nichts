use crate::{Cli, Operation, command::execute_command, error::Error};

pub struct Search;

impl Operation for Search {
  fn operate(cli: &Cli) -> Result<(), Error> {
    if cli.packages.is_empty() {
      return Err(Error::NotSpecified { kind: "pattern(s)".to_string() });
    }

    let flake = cli.flake_url();
    let as_json = cli.json.then(|| "--json").unwrap_or("");

    execute_command(format!("nix search {flake} {as_json} -- {query}", query = &cli.packages.join(" ")), false)
  }
}
