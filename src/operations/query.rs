use crate::{ Operation, command::{exec_cmd, prepare_cmd}, operations::Operations, options::Options };

fn strip_ansi(str: impl Into<String>) -> String {
  let str = str.into();
  let str = str.as_str()
    .replace("\u{1b}[1m", "")
    .replace("\u{1b}[0m", "")
    .trim()
    .to_string();

  str
}

fn pattern_match(values: &mut Vec<String>, patterns: &Vec<String>) {
  for pattern in patterns {
    values.retain(|pkg| pkg.contains(pattern));
  }
}

pub struct Query;

impl Operation for Query {
  fn operate(cli: &crate::Cli) -> Result<(), (i32, &str)> {
    if cli.help {
      Options::print_help(Operations::Query);
      return Ok(());
    }

    if cli.info {
      if !cli.search || cli.packages.is_empty() {
        return exec_cmd("nix profile list", false);
      }

      let list = prepare_cmd(vec!["nix", "profile", "list"], true)
        .output()
        .expect("Failed to list packages");

      let list = String::from_utf8_lossy(&list.stdout).to_string();

      let mut packages = Vec::new();
      let mut package = String::new();

      for line in list.lines() {
        if line.is_empty() {
          packages.push(package.clone());
          package.clear();
        } else {
          package.push_str(line);
          package.push('\n');
        }
      }

      if !package.is_empty() {
        packages.push(package.clone());
      }

      if packages.is_empty() {
        return Err((1, "no package(s) found"));
      }

      if cli.search {
        pattern_match(&mut packages, &cli.packages);
      }

      print!("{}", packages.join("\n"));

      return Ok(());
    }

    let list = prepare_cmd(vec!["nix", "profile", "list"], true)
      .output()
      .expect("Failed to list packages");

    let mut list = String::from_utf8_lossy(&list.stdout).to_string();

    let mut package_names = Vec::new();

    for line in list.lines() {
      if line.contains("Name") {
        if let Some(last_field) = line.split_whitespace().last() {
          let last_field = strip_ansi(last_field);
          package_names.push(last_field);
        }
      }
    }

    if cli.search {
      if cli.packages.is_empty() {
        return Err((1, "no pattern(s) specified"));
      }

      pattern_match(&mut package_names, &cli.packages);
    }

    if package_names.is_empty() {
      return Err((1, "no package(s) found"));
    }

    list = package_names.join("\n");

    println!("{list}");

    return Ok(());
  }
}
