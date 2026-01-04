use crate::{ Operation, command::{catch_output, execute_command}, error::Error, operations::Operations, options::Options };
use fast_strip_ansi::strip_ansi_string;

pub fn pattern_match(values: &mut Vec<String>, patterns: &Vec<String>) {
  values.retain(|pkg| {
    let name = pkg.lines()
      .find(|line| line.starts_with("Name:"))
      .and_then(|line| line.split_whitespace().last())
      .unwrap_or("");

    if name.is_empty() {
      return patterns.iter().any(|pattern| pkg.contains(pattern));
    }

    let flake_attr = pkg.lines()
      .find(|line| line.starts_with("Flake attribute:"))
      .and_then(|line| line.split_whitespace().last())
      .unwrap_or("");

    if flake_attr.is_empty() {
      return false;
    }

    patterns.iter().any(|pattern| {
      name.contains(pattern) || flake_attr.contains(pattern)
    })
  });
}

pub fn list_packages(with_info: bool, profile: String) -> Vec<String> {
  let list_output = catch_output(format!("nix profile list {profile}"), true);
  if let Err(_) = list_output {
    return vec![];
  }

  let list = list_output.unwrap();

  let mut packages = Vec::new();
  let mut package = String::new();

  for line in list.lines() {
    if with_info {
      if line.is_empty() {
        packages.push(package.clone());
        package.clear();
      } else {
        package.push_str(line);
        package.push('\n');
      }
    } else {
      if line.contains("Name") {
        if let Some(last_field) = line.split_whitespace().last() {
          let last_field = strip_ansi_string(last_field).to_string();
          packages.push(last_field);
        }
      }
    }
  }

  if with_info && !package.is_empty() {
    packages.push(package.clone());
  }

  packages
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

    if cli.info {
      if !cli.search || cli.packages.is_empty() {
        return execute_command(format!("nix profile list {profile}"), false);
      }
    }

    let mut packages = list_packages(cli.info, profile);

    if cli.search {
      if cli.packages.is_empty() {
        return Err(Error::NotSpecified { kind: "pattern(s)".to_string() });
      }

      pattern_match(&mut packages, &cli.packages);
    }

    if packages.is_empty() {
      return Err(Error::NoPackageFound);
    }

    println!("{}", packages.join("\n"));

    Ok(())
  }
}
