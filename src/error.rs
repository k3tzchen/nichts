use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
  Unknown { code: i32, message: String },
  NotSpecified { kind: String },
  CommandFailed { code: i32 },
  FailedRollback,
  NoPackageFound,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      Error::Unknown { code: _, message } => write!(f, "{message}"),
      Error::FailedRollback => write!(f, "failed to rollback versions"),
      Error::NoPackageFound => write!(f, "no package(s) found"),
      Error::NotSpecified { kind } => write!(f, "no {kind} specified"),
      _ => write!(f, "")
    }
  }
}

impl Error {
  pub fn exit_code(&self) -> i32 {
    match self {
      Error::CommandFailed { code } => *code,
      Error::Unknown { code, message: _ } => *code,
      _ => 1,
    }
  }
}

impl std::error::Error for Error {}
