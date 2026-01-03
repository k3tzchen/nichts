use crate::{Cli, Operation, command::exec_cmd};

pub struct Search;

impl Operation for Search {
  fn operate(cli: &Cli) -> Result<(), (i32, &str)> {

    let flake = if cli.flake.is_some() {
      format!("{}", cli.flake.clone().unwrap())
    } else {
      "nixpkgs".to_string()
    };

    return exec_cmd(format!("nix search {flake} -- {}", &cli.packages.join(" ")), false);
  }
}
