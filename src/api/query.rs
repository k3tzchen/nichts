use std::{collections::{BTreeMap, btree_map::Keys}, fs::{metadata, read_dir}, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Cli, command::catch_output, error::Error, operations::Operations};

fn verion_default() -> String {
  "latest".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
  pub active: bool,
  #[serde(rename = "attrPath")]
  pub attr_path: String,
  #[serde(rename = "originalUrl")]
  pub original_url: String,
  pub outputs: Option<String>,
  pub priority: isize,
  #[serde(rename = "storePaths")]
  pub store_paths: Vec<String>,
  pub url: String,

  pub homepage: Option<String>,
  pub installed_size: Option<u64>,
  #[serde(default = "verion_default")]
  pub version: String,
}

fn get_dir_size(path: &Path) -> io::Result<u64> {
  let mut size = 0;
  for entry in read_dir(path)? {
    let entry = entry?;
    let path = entry.path();

    size += match () {
      _ if path.is_file() => metadata(&path)?.len(),
      _ if path.is_dir() => get_dir_size(&path)?,
      _ => 0
    };
  }
  Ok(size)
}

impl Package {
  fn store_path(&self) -> Option<String> {
    let mut paths = self.store_paths.clone();
    if !self.store_paths.len().eq(&1) {
      paths.retain(|path| !path.ends_with("-man"));
    }

    if let Some(store_path) = paths.get(0) {
      return Some(store_path.clone());
    }

    None
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageListing {
  elements: BTreeMap<String, Package>
}

impl PackageListing {
  pub fn new(cli: &Cli) -> Self {
    let json_output = catch_output(cli.prepare_command("nix profile list --json"), false);
    if let Err(err) = json_output {
      Operations::throw_if_needed(Err(err));
    }
    let json_output = json_output.unwrap();

    let listing = serde_json::from_str::<PackageListing>(&json_output);
    if let Err(err) = listing {
      Operations::throw_if_needed(Err(Error::Unknown { code: 1, message: err.to_string() }));
    }

    let mut listing = listing.unwrap();

    listing.elements.iter_mut().for_each(|(key, package)| {
      let mut paths = package.store_paths.clone();
      if !package.store_paths.len().eq(&1) {
        paths.retain(|path| !path.ends_with("-man"));
      }

      if let Some(store_path) = package.store_path() {
        let path = Path::new(&store_path);

        if let Some(filename) = path.file_name() {
          if let Some(filename_str) = filename.to_str() {
            if let Some(index) = filename_str.find(key) {
              let version = &filename_str[(index.saturating_add(key.len()))..];

              if !version.len().eq(&0) {
                package.version = version[1..].to_string();
              }
            }
          }
        }

        let dir_size = get_dir_size(path);
        if let Ok(size) = dir_size {
          package.installed_size = Some(size);
        }
      }
    });

    listing
  }

  pub fn retain<P: FnMut(&str, &Package) -> bool>(&mut self, mut predicate: P) -> &mut Self {
    self.elements.retain(|key, value| !predicate(key, value));
    self
  }

  pub fn keys(&self) -> Keys<'_, String, Package> {
    self.elements.keys()
  }

  pub fn to_vec(&self) -> Vec<(&String, &Package)> {
    self.elements.iter().collect()
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  #[allow(dead_code)]
  pub fn any<P: Fn(&String, &Package) -> bool>(&self, predicate: P) -> bool {
    self.elements.iter().any(|(key, value)| predicate(key, value))
  }

  pub fn any_mut<P: FnMut(&String, &Package) -> bool>(&mut self, mut predicate: P) -> bool {
    self.elements.iter_mut().any(|(key, value)| predicate(key, value))
  }

  #[allow(dead_code)]
  pub fn every<P: Fn(&String, &Package) -> bool>(&self, predicate: P) -> Vec<(&String, &Package)> {
    self.elements.iter().filter(|(key, value)| !predicate(key, value)).collect()
  }

  #[allow(dead_code)]
  pub fn for_each<P: Fn(&String, &Package) -> ()>(&self, predicate: P) -> () {
    self.elements.iter().for_each(|(key, value)| {
      predicate(key, value);
    });
  }
}
