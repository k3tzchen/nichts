use crate::{
  Operation,
  api::query::PackageListing,
  error::Error,
  operations::Operations,
  options::Options
};

static UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
fn format_size(bytes: u64) -> String {
  let mut size = bytes as f64;
  let mut unit_idx = 0;

  while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
    size /= 1024.0;
    unit_idx += 1;
  }

  format!("{:.2} {}", size, UNITS[unit_idx])
}

pub struct Query;

impl Operation for Query {
  fn operate(cli: &crate::Cli) -> Result<(), Error> {
    Options::validate_options(&cli, Operations::Query)?;

    let mut listing = PackageListing::new(&cli);

    if cli.search {
      if cli.packages.is_empty() {
        return Err(Error::NotSpecified { kind: "pattern(s)".to_string() });
      }

      listing.retain(|key, _| !cli.packages.iter().any(|keyword| key.contains(keyword)));
    }

    if let Some(flake) = &cli.flake {
      listing.retain(|_, package| !package.original_url.eq(flake));
    }

    if cli.json {
      if let Ok(serialized) = serde_json::to_string_pretty(&listing) {
        println!("{serialized}");
        return Ok(());
      }

      return Err(Error::FailedJsonSerialization);
    }

    if listing.is_empty() {
      return Err(Error::NoPackageFound);
    }

    if cli.info {
      listing.retain(|_, package| !package.active);
      if listing.is_empty() {
        return Err(Error::NoPackageFound);
      }

      let padding = vec!["Name", "Flake attribute", "Flake Url", "Version", "Homepage", "Installed Size", "Store Paths"].iter().map(|str| str.len()).max().unwrap_or(0);

      let print_info = |key: &str, value: &str| {
        println!("{key}{} : {value}", " ".repeat(padding.saturating_sub(key.len())));
      };

      for (name, package) in listing.to_vec() {
        print_info("Name", &name);
        print_info("Flake attribute", &package.attr_path);
        print_info("Flake Url", &package.original_url);
        print_info("Version", &package.version);

        if let Some(homepage) = &package.homepage {
          print_info("Homepage", &homepage);
        }
        let installed_size = package.installed_size.unwrap_or(0);
        print_info("Installed Size", &format_size(installed_size));
        print_info("Store Paths", &package.store_paths.join(" "));
        println!("");
      }

      return Ok(());
    }

    if cli.quiet {
      listing.keys().for_each(|key| println!("{key}"));
      return Ok(());
    }

    listing.to_vec().iter().for_each(|(name, package)| println!("{name} {version}", version = package.version));

    Ok(())
  }
}
