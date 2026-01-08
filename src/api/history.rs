use std::{collections::{BTreeMap, btree_map::Iter}, fmt::Display};
use serde::{Deserialize, Serialize};

use crate::{Cli, command::catch_output, error::Error, operations::Operations};
use fast_strip_ansi::strip_ansi_string;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum HistoryActions {
  Added,
  Removed,
  Upgraded,
  None
}

impl Display for HistoryActions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     match self {
       HistoryActions::Added => write!(f, "added"),
       HistoryActions::Removed => write!(f, "removed"),
       HistoryActions::Upgraded => write!(f, "upgraded"),
       _ => write!(f, "_")
     }
  }
}

impl Clone for HistoryActions {
  fn clone(&self) -> Self {
    match self {
      HistoryActions::Added => HistoryActions::Added,
      HistoryActions::Removed => HistoryActions::Removed,
      HistoryActions::Upgraded => HistoryActions::Upgraded,
      HistoryActions::None => HistoryActions::None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryPackage {
  pub action: HistoryActions,
  pub previous_version: Option<String>,
  pub current_version: Option<String>,
  pub flake_url: String,
  pub flake_attribute: String
}

pub static UNSET_VERSION: &str = "∅";
static EPSILON_VERSION: &str = "ε";

impl HistoryPackage {
  #[allow(dead_code)]
  fn flake(&self) -> String {
    format!("{url}#{attribute}", url = self.flake_url, attribute = self.flake_attribute)
  }
}

impl From<String> for HistoryPackage {
  fn from(value: String) -> Self {
    if value.rfind("->").is_none() {
      return HistoryPackage { action: HistoryActions::None, previous_version: None, current_version: None, flake_url: String::new(), flake_attribute: String::new() };
    }

    let mut parts = value.split(": ");

    let mut flake_url = String::new();
    let mut flake_attribute = String::new();

    if let Some(flake) = parts.next() {
      let mut url_and_attribute = flake.split("#");

      if let Some(url) = url_and_attribute.next() {
        flake_url.push_str(url);
      }

      if let Some(attribute) = url_and_attribute.next() {
        flake_attribute.push_str(attribute);
      }
    }

    let mut action = HistoryActions::Added;

    let mut previous_version = None;
    let mut current_version = None;

    let denotation = parts.collect::<Vec<&str>>().join(":");
    let denotation = denotation.trim();

    let mut version_parts = denotation.split("->");

    if let Some(previous) = version_parts.next() {
      let previous = previous.trim();
      if !previous.eq(UNSET_VERSION) {
        previous_version.replace(previous.to_string());
      }
    }

    if let Some(current) = version_parts.next() {
      let current = current.trim();
      if current.eq(UNSET_VERSION) {
        action = HistoryActions::Removed;
      } else {
        if !previous_version.is_none() {
          action = HistoryActions::Upgraded;
        }

        let mut current_version_list = current.split(",");
        if let Some(primary_version) = current_version_list.next() {
          current_version.replace(
            if primary_version.eq(EPSILON_VERSION) {
              "latest".to_string()
            } else {
              primary_version.trim().to_string()
            }
          );
        }
      }
    }

    HistoryPackage { action, previous_version, current_version, flake_url, flake_attribute }
  }
}

impl From<&str> for HistoryPackage {
  fn from(value: &str) -> Self {
    return HistoryPackage::from(value.to_string());
  }
}

impl Clone for HistoryPackage {
  fn clone(&self) -> Self {
    HistoryPackage {
      action: self.action.clone(),

      previous_version: self.previous_version.clone(),
      current_version: self.current_version.clone(),

      flake_url: self.flake_url.clone(),
      flake_attribute: self.flake_attribute.clone(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct HistoryVersions {
  versions: BTreeMap<usize, Vec<HistoryPackage>>,
}

impl HistoryVersions {
  pub fn new(cli: &Cli) -> Self {
    let mut version_map = BTreeMap::new();

    let profile = cli.profile.clone().map(|profile| {
      if !profile.is_empty() {
        return format!("--profile {profile}");
      }

      return profile;
    }).unwrap_or_else(|| "".to_string());

    let history_output = catch_output(format!("nix profile history {profile}"), true);
    if let Err(err) = history_output {
      Operations::throw_if_needed(Err(err));
    }
    let history_output = history_output.unwrap();

    let mut current_version_index: usize = 0;
    let mut current_version_list = Vec::new();

    for line in history_output.lines() {
      let line = strip_ansi_string(line.trim()).to_string();

      if line.is_empty() {
        continue;
      }

      if line.starts_with("Version") {
        let version_number = line.split_whitespace().skip(1).next();
        if let Some(current_version) = version_number {
          let current_version = current_version.parse::<usize>();
          if let Ok(version_number) = current_version {

            let previous_version = current_version_list.iter().cloned();
            let previous_version = previous_version.collect::<Vec<HistoryPackage>>();

            version_map.insert(current_version_index, previous_version);
            if !current_version_list.is_empty() {
              current_version_list.clear();
            }

            current_version_index = version_number;
          } else {
            Operations::throw_if_needed(Err(Error::Unknown {
              code: 1,
              message: format!("failed to parse version number: {version_number:?}")
            }));
          }
        } else {
          break;
        }

        continue;
      }

      let package = HistoryPackage::from(line);

      if !package.action.eq(&HistoryActions::None) {
        current_version_list.push(package);
      }
    }

    if !current_version_list.is_empty() {
      let previous_version = current_version_list.iter().cloned();
      let previous_version = previous_version.collect::<Vec<HistoryPackage>>();

      version_map.insert(current_version_index, previous_version);
      if !current_version_list.is_empty() {
        current_version_list.clear();
      }
    }

    HistoryVersions { versions: version_map }
  }

  pub fn iter(&self) -> Iter<'_, usize, Vec<HistoryPackage>> {
    self.versions.iter()
  }

  #[allow(dead_code)]
  pub fn get(&self, version: usize) -> Option<&Vec<HistoryPackage>> {
    self.versions.get(&version)
  }
}
