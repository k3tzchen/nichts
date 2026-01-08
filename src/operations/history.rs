use super::{Operation, Operations};
use crate::{Cli, api::history::{HistoryVersions, UNSET_VERSION}, command::{confirm, execute_command}, error::Error, options::{Options, clean::Clean}};

pub struct History;

impl Operation for History {
  fn operate(cli: &Cli) -> Result<(), Error> {
    Options::validate_options(&cli, Operations::History)?;

    if cli.wipe.is_some() {
      if !cli.noconfirm && !confirm("Do you want to wipe your history?") {
        return Err(Error::Unknown { code: 1, message: "".to_string() });
      }

      let wipe_time = cli.wipe.clone().unwrap();

      let mut older_than = String::new();
      if !wipe_time.is_empty() {
        older_than = format!(" --older-than {wipe_time}");
      }

      let command = cli.prepare_command("nix profile wipe-history");
      execute_command(format!("{command}{older_than}"), false)?;

      return Ok(());
    }

    let versions = HistoryVersions::new(&cli);
    if cli.packages.is_empty() {
      if cli.rollback {
        return Err(Error::NotSpecified { kind: "generation".to_string() });
      }

      if cli.clean {
        return Err(Error::Unknown { code: 1, message: "cannot use '--clean' in current state (use -h for help)".to_string() });
      }

      if cli.json {
        if let Ok(serialized) = serde_json::to_string_pretty(&versions) {
          println!("{serialized}");
          return Ok(());
        }

        return Err(Error::FailedJsonSerialization);
      }

      for (version, changes) in versions.iter() {
        println!("Version {version}:");
        if changes.is_empty() {
          println!("  No changes.");
        } else {
          for change in changes {
            println!("  {flake}#{attribute}: {previous} -> {latest}",
              flake = change.flake_url, attribute = change.flake_attribute,
              latest = change.current_version.clone().unwrap_or(UNSET_VERSION.to_string()), previous = change.previous_version.clone().unwrap_or(UNSET_VERSION.to_string())
            );
          }
        }
        println!("");
      }

      return Ok(());
    }

    if let Some(arg0) = cli.packages.get(0) {
      let arg0_unsigned = arg0.clone().parse::<usize>();
      if let Err(_) = arg0_unsigned {
        return Err(Error::Unknown { code: 1, message: "invalid version number".to_string() });
      }
      let arg0_unsigned = arg0_unsigned.unwrap();

      if cli.rollback {
        if !cli.noconfirm && !confirm("Do you want to rollback?") {
          return Err(Error::Unknown { code: 1, message: "".to_string() });
        }

        let command = cli.prepare_command("nix profile rollback");
        execute_command(format!("{command} --to {arg0}"), false)?;

        if cli.clean {
          Clean::operate(&cli)?;
        }

        return Err(Error::FailedRollback);
      }

      if cli.json {
        if let Some(version_package) = versions.get(arg0_unsigned) {
          if let Ok(serialized) = serde_json::to_string_pretty(&version_package) {
            println!("{serialized}");
            return Ok(());
          }
        }

        return Err(Error::FailedJsonSerialization);
      }

      for (_, changes) in versions.iter().filter(|(version_number, _)| version_number.eq(&&arg0_unsigned)) {
        if changes.is_empty() {
          println!("No changes.");
        } else {
          for change in changes {
            println!("{flake}#{attribute}: {previous} -> {latest}",
              flake = change.flake_url, attribute = change.flake_attribute,
              latest = change.current_version.clone().unwrap_or(UNSET_VERSION.to_string()), previous = change.previous_version.clone().unwrap_or(UNSET_VERSION.to_string())
            );
          }
        }
      }
    }

    return Ok(());
  }
}
