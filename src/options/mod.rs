use std::{fmt::Display};

use crate::{CLI_NAME, Cli, error::Error, operations::Operations};

#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
pub enum Options {
  Clean,
  Flake,
  Impure,
  Info,
  Json,
  NoConfirm,
  Profile,
  Quiet,
  Refresh,
  Rollback,
  Search,
  Upgrade,
  Wipe,
}

impl Ord for Options {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.to_string().cmp(&other.to_string())
  }
}

impl Options {
  fn all() -> &'static [Options] {
    &[
      Options::Clean,
      Options::Flake,
      Options::Impure,
      Options::Info,
      Options::Json,
      Options::NoConfirm,
      Options::Profile,
      Options::Quiet,
      Options::Refresh,
      Options::Rollback,
      Options::Search,
      Options::Upgrade,
      Options::Wipe,
    ]
  }

  fn partial(operation: &Operations) -> &'static [Options] {
    match operation {
      Operations::Query => &[Options::Info, Options::Flake, Options::Json, Options::Profile, Options::Quiet, Options::Search],
      Operations::Remove => &[Options::Clean, Options::Flake, Options::NoConfirm, Options::Profile, Options::Quiet],
      Operations::History => &[Options::Clean, Options::Json, Options::NoConfirm, Options::Profile, Options::Quiet, Options::Rollback, Options::Wipe],
      Operations::Sync => &[Options::Clean, Options::Flake, Options::Impure, Options::Json, Options::NoConfirm, Options::Profile, Options::Quiet, Options::Refresh, Options::Search, Options::Upgrade],
      _ => &[]
    }
  }

  fn get_conflicts(operation: &Operations) -> &'static [(Options, &'static [Options])] {
    match operation {
      Operations::History => &[
        (Options::Json, &[Options::Wipe, Options::Rollback, Options::Clean]),
      ],
      Operations::Query => &[
        (Options::Json, &[Options::Info, Options::Quiet])
      ],
      _ => &[]
    }
  }

  pub fn validate_options(cli: &Cli, operation: Operations) -> Result<(), Error> {
    if cli.help {
      Options::print_help(Operations::Query);
      return Ok(());
    }

    let set = &[
      (Options::Clean, cli.clean),
      (Options::Flake, cli.flake.is_some()),
      (Options::Impure, cli.impure),
      (Options::Info, cli.info),
      (Options::Json, cli.json),
      (Options::NoConfirm, cli.noconfirm),
      (Options::Profile, cli.profile.is_some()),
      (Options::Quiet, cli.quiet),
      (Options::Refresh, cli.refresh),
      (Options::Rollback, cli.rollback),
      (Options::Search, cli.search),
      (Options::Upgrade, cli.upgrade),
      (Options::Wipe, cli.wipe.is_some()),
    ];

    let allowed_options = Options::partial(&operation).to_vec();
    let confilcts = Options::get_conflicts(&operation);

    for (option, is_set) in set {
      if *is_set {
        if !allowed_options.contains(&option) {
          return Err(Error::InvalidOption {
            option: format!("--{}", option.long()),
            conflicts_with: None
          });
        }

        if let Some((_, conflicts_with)) = confilcts.iter().find(|(opt, _)| opt == option) {
          if let Some(conflict) = conflicts_with.iter().find(|c| set.iter().any(|(opt, is_set_val)| &opt == c && *is_set_val)) {
            return Err(Error::InvalidOption {
              option: format!("--{}", option.long()),
              conflicts_with: Some(conflict.long().to_string())
            });
          }
        }
      }
    }


    Ok(())
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
      Options::Json => "json",
      Options::Refresh => "refresh",
      Options::Clean => "clean",
      Options::Quiet => "quiet",
      Options::Profile => "profile",
      Options::Wipe => "wipe",
      Options::NoConfirm => "noconfirm",
      Options::Rollback => "rollback",
    }
  }

  pub fn arguments(&self) -> &str {
    match self {
      Options::Flake => "<path>",
      Options::Profile => "<path>",
      Options::Search => "<pattern(s)>",
      Options::Wipe => "[age:<N>d]",
      _ => ""
    }
  }

  fn description(&self) -> &str {
    match self {
      Options::Clean => "delete unreachable store objects",
      Options::Flake => "specify a new default flake to install packages from",
      Options::Impure => "allow access to mutable paths and repositories",
      Options::Info => "display useful metadata",
      Options::Json => "produces output in JSON format",
      Options::NoConfirm => "do not ask for any confirmation",
      Options::Profile => "the profile to operate on",
      Options::Quiet => "decrease the logging output",
      Options::Refresh => "consider all previously downloaded files out-of-date",
      Options::Rollback => "roll back to another version",
      Options::Search => "search for packages matching patterns",
      Options::Upgrade => "upgrade all installed packages",
      Options::Wipe => "delete non-current versions older than the specified age",
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
