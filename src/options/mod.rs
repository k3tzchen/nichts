use std::fmt::Display;

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
    println!("usage: {CLI_NAME} {operation}");

    let options = Options::partial(&operation);
    if options.is_empty() {
      return;
    }

    println!("\noptions:");
    for option in options {
      println!("  {option}");
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

impl Display for Options {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let arguments = self.arguments();
    let short_char = self.short();

    let mut short = "  ";
    let option = format!("-{short_char}");

    if !short_char.eq(&' '){
      short = option.as_str();
    };

    if arguments.is_empty() {
      return write!(f, "{short} --{long}", long = self.long());
    }

    write!(f, "{short} --{long} {arguments}", long = self.long())
  }
}

pub mod clean;
pub mod upgrade;
pub mod search;
