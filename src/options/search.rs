use crate::{Cli, Operation, command::execute_command, error::Error};

pub struct Search;

impl Operation for Search {
  fn operate(cli: &Cli) -> Result<(), Error> {

    let flake = if cli.flake.is_some() {
      format!("{}", cli.flake.clone().unwrap())
    } else {
      "nixpkgs".to_string()
    };

    execute_command(format!("nix search {flake} -- {query}", query = &cli.packages.join(" ")), false)
  }
}
