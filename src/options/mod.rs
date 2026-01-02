use crate::{CLI_NAME, operations::Operations};

pub enum Options {
  Upgrade,
  Flake,
  Search,
  Info,
  Impure,
  Refresh,
  Clean
}

impl Options {
  fn partial(operation: &Operations) -> &'static [Options] {
    match operation {
      Operations::Query => &[Options::Info, Options::Search],
      Operations::Remove | Operations::History => &[Options::Clean],
      Operations::Sync => &[Options::Clean, Options::Flake, Options::Impure, Options::Refresh, Options::Search, Options::Upgrade],
      _ => &[]
    }
  }

  pub fn print_help(operation: Operations) {
    println!("usage: {CLI_NAME} {}", operation.to_string());

    let options = Options::partial(&operation);
    if options.is_empty() {
      return;
    }

    println!("\noptions:");
    for operation in options {
      println!("  {}", operation.to_string());
    }
  }

  pub fn short(&self) -> char {
    match self {
      Options::Clean => 'c',
      Options::Refresh => 'y',
      Options::Search => 's',
      Options::Upgrade => 'u',
      Options::Info => 'i',
      _ => ' '
    }
  }

  pub fn long(&self) -> &str {
    match self {
      Options::Upgrade => "upgrade",
      Options::Flake => "flake",
      Options::Search => "search",
      Options::Info => "info",
      Options::Impure => "impure",
      Options::Refresh => "refresh",
      Options::Clean => "clean"
    }
  }

  pub fn arguments(&self) -> &str {
    match self {
      Options::Flake => "<path>",
      Options::Search => "<pattern(s)>",
      _ => ""
    }
  }
}

impl ToString for Options {
  fn to_string(&self) -> String {
    let short = self.short();
    let argument = format!("-{short}");

    format!("{short} --{long} {arguments}", short = if short.eq(&' ') { "  " } else { argument.as_str()  },  long = self.long(), arguments = self.arguments())
  }
}

pub mod clean;
pub mod upgrade;
pub mod search;
