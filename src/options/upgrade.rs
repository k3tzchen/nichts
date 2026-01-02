use crate::{Operation, command::{exec_cmd, prepare_cmd}, options::clean::Clean};

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
      let output = prepare_cmd(vec!["sh", "-c"], true)
        .arg("nix profile list | grep Name | awk '{print $NF}'")
        .output()
        .expect("Failed to list packages");

      let output = String::from_utf8_lossy(&output.stdout);


      let output = output
        .replace("\u{1b}[1m", "")
        .replace("\u{1b}[0m", "")
        .trim()
        .to_string();

      packages = output
        .lines()
        .collect::<Vec<&str>>()
        .join(" ");
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
