use crate::{
  DEFAULT_FLAKE, Operation, api::query::PackageListing, command::{
    catch_output, confirm, execute_command
  }, error::Error, operations::Operations, options::{
    Options,
    clean::Clean,
    search::Search,
    upgrade::Upgrade
  }
};

pub struct Sync;

impl Operation for Sync {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    Options::validate_options(&cli, Operations::Sync)?;

    if cli.upgrade {
      return Upgrade::operate(&cli);
    }

    let mut packages = cli.packages.clone();
    if cli.search {
      return Search::operate(&cli);
    }

    if cli.json {
      return Err(Error::Unknown { code: 1, message: "cannot use '--json' in current state (use -h for help)".to_string() });
    }

    let flake_url = cli.flake_url();

    if packages.is_empty() {
      if flake_url.eq(DEFAULT_FLAKE) {
        return Err(Error::NotSpecified { kind: "target(s)".to_string() });
      }

      packages.push("default".to_string());
    }

    let mut installed_packages = PackageListing::new(&cli);
    let installed_packages_keys: Vec<String> = installed_packages.keys().map(|key| key.clone()).collect();

    let mut print_seperator_line = false;
    let new_packages: Vec<String> = packages.iter().map(|package| {
      let mut package_format = package.clone();

      if !installed_packages_keys.contains(&package) {
        let evaluate = catch_output(format!("nix eval --raw -- {flake_url}#{package}.version"), false);
        if let Ok(version) = evaluate {
          return format!("{package}-{version}");
        }

        Operations::throw_if_needed(Err(Error::Unknown { code: 1, message: format!("flake '{flake_url}' does not provide attribute '{package}'")}));
      }

      let has_installed_packages = installed_packages.any_mut(|name, installed_package| {
        if !name.eq(package.as_str()) {
          return false;
        }

        if cli.flake.is_some() && installed_package.original_url != flake_url {
          Operations::throw_if_needed(Err(Error::Unknown { code: 1, message: format!("flake '{flake_url}' does not provide attribute '{package}'")}));
        }

        let evaluate = catch_output(format!("nix eval --raw --offline -- {flake}#{attribute}.version", flake = installed_package.original_url, attribute = installed_package.attr_path), false);
        if let Err(err) = evaluate {
          Operations::throw_if_needed(Err(err));
        }

        let latest_version = evaluate.unwrap();
        package_format = format!("{name}-{latest_version}");

        if installed_package.version.ends_with(&latest_version) && !cli.refresh {
          Operations::show_warning(format!("{package_format} is already up to date"));
        }

        package_format = format!("{flake}#{attribute}-{latest_version}", flake = installed_package.original_url, attribute = installed_package.attr_path);

        return true;
      });

      if !print_seperator_line {
        print_seperator_line = has_installed_packages;
      }

      package_format
    }).collect();

    if packages.is_empty() {
      return Err(Error::NotSpecified { kind: "target(s)".to_string() });
    }

    if print_seperator_line {
      println!("");
    }

    println!("Packages ({length}) {new_packages_list}\n", length = new_packages.len(), new_packages_list = new_packages.join(" "));

    if !cli.noconfirm && !confirm("Proceed with installation?") {
      return Err(Error::Unknown { code: 1, message: "".to_string() });
    }

    let command = cli.prepare_command("nix profile add");
    for package in packages  {
      execute_command(format!("{command} -- {flake_url}#{package}"), false)?;
    }

    if cli.clean {
      Clean::operate(&cli)?;
    }

    Ok(())
  }
}
