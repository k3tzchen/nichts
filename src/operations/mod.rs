use std::{fmt::Display, process::exit};

use crate::{ CLI_NAME, Cli };

pub enum Operations {
  Sync,
  Remove,
  Query,
  Version,
  Help,
  History
}

impl Operations {
  fn all() -> &'static [Operations] {
    &[
      Operations::Help,
      Operations::History,
      Operations::Query,
      Operations::Remove,
      Operations::Sync,
      Operations::Version,
    ]
  }

  fn print_help() {
    for operation in Operations::all() {
      println!("  {CLI_NAME} {operation}");
    }
  }

  pub fn short(&self) -> char {
    match self {
      Operations::Sync => 'S',
      Operations::Remove => 'R',
      Operations::Query => 'Q',
      Operations::Version => 'V',
      Operations::History => 'H',
      Operations::Help => 'h'
    }
  }

  pub fn long(&self) -> &str {
    match self {
      Operations::Sync => "sync",
      Operations::Remove => "remove",
      Operations::Query => "query",
      Operations::Version => "version",
      Operations::History => "history",
      Operations::Help => "help"
    }
  }

  pub fn arguments(&self) -> &str {
    match self {
      Operations::Sync | Operations::Query => "[options] <package(s)>",
      Operations::Remove => "<package(s)>",
      Operations::History => "[options] <rollback generation>",
      _ => ""
    }
  }

  pub fn assert(res: Result<(), (i32, &str)>) -> ! {
    if let Err((code, message)) = res {
      if !message.is_empty() {
        eprintln!("Error: {message}");
      }
      exit(code);
    }

    exit(0);
  }
}

impl Display for Operations {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let arguments = self.arguments();

    if arguments.is_empty() {
      return write!(f, "{{-{short} --{long}}}", short = self.short(), long = self.long());
    }

    write!(f, "{{-{short} --{long}}} {arguments}", short = self.short(), long = self.long())
  }
}

pub(super) trait Operation {
  fn operate(cli: &Cli) -> Result<(), (i32, &str)>;
}

pub mod help;
pub mod history;
pub mod remove;
pub mod version;
pub mod sync;
pub mod query;
