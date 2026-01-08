use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
  Unknown { code: i32, message: String },
  UnknownOption { option: String },
  InvalidOption { option: String, conflicts_with: Option<String> },
  NotSpecified { kind: String },
  CommandFailed { code: i32 },
  FailedRollback,
  FailedJsonSerialization,
  NoPackageFound,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      Error::Unknown { code: _, message } => write!(f, "{message}"),
      Error::FailedRollback => write!(f, "failed to rollback versions"),
      Error::NoPackageFound => write!(f, "no package(s) found"),
      Error::FailedJsonSerialization => write!(f, "failed to serialize output"),
      Error::NotSpecified { kind } => write!(f, "no {kind} specified (use -h for help)"),
      Error::UnknownOption { option } => write!(f, "unrecognized option '{option}' (use -h for help)"),
      Error::InvalidOption { option, conflicts_with } => {
        let mut info_text = "(use -h for help)".to_string();
        if let Some(conflict_option) = conflicts_with.as_deref() {
          info_text = format!("and '--{conflict_option}' may not be used together");
        }

        write!(f, "invalid option: '{option}' {info_text}")
      },
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
