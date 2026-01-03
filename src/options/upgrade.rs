use crate::{Operation, command::exec_cmd, operations::query::list_packages, options::clean::Clean};

pub struct Upgrade;

impl Operation for Upgrade {
  fn operate(cli: &crate::Cli) -> Result<(), (i32, &str)> {
    let mut packages = cli.packages.join(" ");
    let mut command = "nix profile upgrade --verbose".to_string();

    if cli.impure {
      command.push_str(" --impure");
    }

    if cli.refresh {
      command.push_str(" --refresh");
    }

    if packages.is_empty() {
      let installed_packages = list_packages(false);
      if installed_packages.is_empty() {
        return Err((1, "no package(s) to upgrade"));
      }
      packages = installed_packages.join(" ");
    }

    if let Err(err) = exec_cmd(format!("{command} -- {packages}"), false) {
      return Err(err);
    }

    if cli.clean {
      return Clean::operate(&cli);
    }

    return Ok(());
  }
}
