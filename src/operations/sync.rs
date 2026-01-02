use crate::{Operation, command::exec_cmd, operations::Operations, options::{Options, clean::Clean, search::Search, upgrade::Upgrade} };

pub struct Sync;

impl Operation for Sync {
  fn operate(cli: &crate::Cli) -> Result<(), (i32, &str)> {
    if cli.help {
      Options::print_help(Operations::Sync);
      return Ok(());
    }

    let packages = &cli.packages;

    if cli.update {
      return Upgrade::operate(&cli);
    }

    if cli.search {
      if cli.packages.is_empty() {
        return Err((1, "no pattern(s) specified"));
      }

      return Search::operate(&cli);
    }

    let mut command = "nix profile add --verbose".to_string();

    let package_prefix =
      if cli.flake.is_some() { format!("{}#", cli.flake.clone().unwrap()) }
      else { "nixpkgs#".to_string() };

    if cli.impure {
      command.push_str(" --impure");
    }

    if cli.refresh {
      command.push_str(" --refresh");
    }

    for package in packages  {
      if let Err(err) = exec_cmd(format!("{command} {package_prefix}{package}"), false) {
        return Err(err);
      }
    }

    if cli.clean {
      return Clean::operate(&cli);
    }

    return Ok(());
  }
}
