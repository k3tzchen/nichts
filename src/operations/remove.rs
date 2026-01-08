use super::{Operation, Operations};
use crate::{ api::query::PackageListing, command::{catch_output, confirm, execute_command}, error::Error, options::{Options, clean::Clean} };

pub struct Remove;

impl Operation for Remove {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    Options::validate_options(&cli, Operations::Remove)?;

    if cli.packages.is_empty() {
      return Err(Error::NotSpecified { kind: "target(s)".to_string() });
    }

    let mut installed_packages = PackageListing::new(&cli);
    let installed_packages_keys: Vec<String> = installed_packages.keys().map(|key| key.clone()).collect();

    let packages = &cli.packages;
    let not_installed_packages: Vec<String> = packages.iter().filter(|key| !installed_packages_keys.contains(key)).map(|key| key.clone()).collect();

    if !not_installed_packages.is_empty() {
      for package in not_installed_packages {
        Operations::show_warning(format!("target not found: {package}"));
      }
      return Err(Error::Unknown { code: 1, message: "".to_string() });
    }

    let flake = cli.flake_url();
    let new_packages: Vec<String> = packages.iter().map(|package| {
      let mut package_format = package.clone();

      let _ = installed_packages.any_mut(|name, installed_package| {
        if !name.eq(package.as_str()) {
          return false;
        }

        if cli.flake.is_some() && installed_package.original_url != flake {
          Operations::throw_if_needed(Err(Error::Unknown { code: 1, message: format!("flake '{flake}' does not provide attribute '{package}'")}));
        }

        let evaluate = catch_output(format!("nix eval --raw --offline -- {flake}#{attribute}.version", flake = installed_package.original_url, attribute = installed_package.attr_path), false);
        if let Err(err) = evaluate {
          Operations::throw_if_needed(Err(err));
        }

        let latest_version = evaluate.unwrap();
        package_format = format!("{name}-{latest_version}");

        return true;
      });

      package_format
    }).collect();

    println!("Packages ({length}) {new_packages_list}\n", length = new_packages.len(), new_packages_list = new_packages.join(" "));

    if !cli.noconfirm && !confirm("Do you want to remove these packages?") {
      return Err(Error::Unknown { code: 1, message: "".to_string() });
    }

    let command = cli.prepare_command("nix profile remove");
    execute_command(format!("{command} -- {}", packages.join(" ")), false)?;

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
