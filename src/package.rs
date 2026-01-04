use std::{
  fmt::Display,
  path::Path
};

use fast_strip_ansi::strip_ansi_string;

use crate::error::Error;

#[derive(Debug)]
pub struct Package {
  unprocessed: String,
  pub name: String,
  pub flake_attribute: String,
  pub flake_url: String,
  pub version: String,
}

fn get_version(path: &str, package_name: &str) -> Option<String> {
  let filename = Path::new(path)
    .file_name()?
    .to_str()?;

  let version = &filename[(filename.find(package_name)?.saturating_add(package_name.len()))..];

  if version.len().eq(&0) {
    return None;
  }

  Some(version[1..].to_string())
}

impl Package {
  pub fn from(str: impl Into<String>) -> Result<Self, Error> {
    let str = str.into();
    let unprocessed = str.to_string();
    let str = strip_ansi_string(&str).to_string();

    let lines = str.lines();

    let mut package = Package {
      unprocessed,
      name: String::new(),
      flake_attribute: "default".to_string(),
      flake_url: String::new(),
      version: "latest".to_string()
    };

    let parse_error = Error::Unknown { code: 1, message: "Parsing failed".to_string() };
    for line in lines {
      let lower_line = line.to_ascii_lowercase();
      if lower_line.starts_with("name:") {
        if let Some(last_field) = line.split_whitespace().last() {
          package.name.push_str(last_field);
        } else {
          return Err(parse_error);
        }
      } else
      if lower_line.starts_with("flake attribute:") {
        if let Some(last_field) = line.split_whitespace().last() {
          package.flake_attribute = last_field.to_string();
        } else {
          return Err(parse_error);
        }
      } else
      if lower_line.starts_with("original flake url:") {
        if let Some(last_field) = line.split_whitespace().last() {
          package.flake_url.push_str(last_field);
        } else {
          return Err(parse_error);
        }
      } else
      if lower_line.starts_with("store paths:") {
        let paths = &line[12..].trim();
        let mut paths = paths.split_whitespace().collect::<Vec<&str>>();

        if paths.len().gt(&1) {
          let mut filtered_paths = Vec::new();
          for path in paths {
            if !path.ends_with("-man") {
              filtered_paths.push(path);
            }
          }
          paths = filtered_paths;
        }

        if let Some(path) = paths.get(0) {
          if let Some(version) = get_version(path, &package.name) {
            package.version = version;
          }
        }

        break;
      }
    }

    return Ok(package);
  }

  pub fn to_version_string(&self) -> String {
    let name = &self.name;
    let version = &self.version;

    return format!("{name} {version}");
  }
}

impl Display for Package {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.unprocessed)
  }
}
