use crate::{ Operation, command::catch_output, error::Error, operations::Operations, options::Options, package::Package };

pub fn pattern_match(values: &mut Vec<Package>, patterns: &Vec<String>) {
  values.retain(|pkg| {
    patterns.iter().any(|pattern| {
      pkg.name.contains(pattern) || pkg.flake_attribute.contains(pattern)
    })
  });
}

pub fn list_packages(profile: String) -> Result<Vec<Package>, Error> {
  let list = catch_output(format!("nix profile list {profile}"), true)?;

  let mut package_strings = Vec::new();
  let mut package = String::new();

  for line in list.lines() {
    if line.is_empty() {
      package_strings.push(package.clone());
      package.clear();
    } else {
      package.push_str(line);
      package.push('\n');
    }
  }

  package_strings.push(package.clone());

  let mut packages = Vec::new();

  for string in package_strings {
    let package = Package::from(string)?;
    packages.push(package);
  }

  return Ok(packages);
}

pub struct Query;

impl Operation for Query {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Query);
      return Ok(());
    }

    let profile = cli.profile.clone().map(|profile| {
      if !profile.is_empty() {
        return format!("--profile {profile}");
      }

      return profile;
    }).unwrap_or_else(|| "".to_string());

    let mut packages = list_packages(profile)?;

    if cli.search {
      if cli.packages.is_empty() {
        return Err(Error::NotSpecified { kind: "pattern(s)".to_string() });
      }

      pattern_match(&mut packages, &cli.packages);
    }

    if packages.is_empty() {
      return Err(Error::NoPackageFound);
    }

    println!("{}", packages.iter().map(|pkg| {
      if cli.info {
        return pkg.to_string();
      }

      if cli.quiet {
        return pkg.name.clone();
      }

      return pkg.to_version_string();
    }).collect::<Vec<String>>().join("\n"));

    Ok(())
  }
}
