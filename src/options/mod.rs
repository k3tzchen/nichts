use std::fmt::Display;

use crate::{CLI_NAME, operations::Operations};

pub enum Options {
  Upgrade,
  Flake,
  Search,
  Info,
  Impure,
  Profile,
  Refresh,
  Clean,
  Quiet
}

impl Options {
  fn all() -> &'static [Options] {
    &[
      Options::Upgrade,
      Options::Flake,
      Options::Search,
      Options::Info,
      Options::Impure,
      Options::Profile,
      Options::Refresh,
      Options::Clean,
      Options::Quiet
    ]
  }

  fn partial(operation: &Operations) -> &'static [Options] {
    match operation {
      Operations::Query => &[Options::Profile, Options::Info, Options::Search],
      Operations::Remove | Operations::History => &[Options::Profile, Options::Clean, Options::Quiet],
      Operations::Sync => &[Options::Profile, Options::Clean, Options::Flake, Options::Impure, Options::Quiet, Options::Refresh, Options::Search, Options::Upgrade],
      _ => &[]
    }
  }

  pub fn print_help(operation: Operations) {
    println!("usage: {CLI_NAME} {usage}", usage = operation.usage());

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
      Options::Quiet => 'q',
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
      Options::Clean => "clean",
      Options::Quiet => "quiet",
      Options::Profile => "profile",
    }
  }

  pub fn arguments(&self) -> &str {
    match self {
      Options::Flake => "<path>",
      Options::Search => "<pattern(s)>",
      _ => ""
    }
  }

  fn description(&self) -> &str {
    match self {
      Options::Clean => "delete unreachable store objects",
      Options::Flake => "specify the flake to install packages from",
      Options::Impure => "allow access to mutable paths and repositories",
      Options::Info => "display useful metadata",
      Options::Profile => "the profile to operate on",
      Options::Quiet => "decrease the logging output",
      Options::Refresh => "consider all previously downloaded files out-of-date",
      Options::Search => "search for packages matching patterns",
      Options::Upgrade => "upgrade all installed packages"
    }
  }

  fn len(&self) -> usize {
    let short_char = self.short();

    let mut short = "   ";
    let option = format!("-{short_char},");

    if !short_char.eq(&' '){
      short = option.as_str();
    };

    let arguments = self.arguments();
    if arguments.is_empty() {
      return format!("{short} --{long}", long = self.long()).len();
    }

    format!("{short} --{long} {arguments}", long = self.long()).len()
  }

  fn max_len() -> usize {
    Options::all()
      .iter().map(|option| option.len())
      .max().unwrap_or_else(|| 0)
  }
}

impl Display for Options {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let arguments = self.arguments();
    let description = self.description();
    let short_char = self.short();

    let mut short = "  ";
    let option = format!("-{short_char}");

    if !short_char.eq(&' '){
      short = option.as_str();
    };

    if description.is_empty() {
      if arguments.is_empty() {
        return write!(f, "{short} --{long}", long = self.long());
      }

      return write!(f, "{short} --{long} {arguments}", long = self.long());
    }

    let padding = Self::max_len() - self.len();
    if arguments.is_empty() {
      return write!(f, "{short} --{long}{padding} {description}", long = self.long(), padding = " ".repeat(padding));
    }

    write!(f, "{short} --{long} {arguments}{padding} {description}", long = self.long(), padding = " ".repeat(padding))
  }
}

pub mod clean;
pub mod upgrade;
pub mod search;
