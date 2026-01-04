use std::process::{ Command, Stdio };

use crate::error::Error;

fn parse_args(command: impl Into<String>) -> Result<Vec<String>, Error> {
  let command = command.into();
  let command = command.trim();

  if command.is_empty() {
    return Err(Error::Unknown { code: 1, message: "Command was empty".to_string() });
  }

  let args = shlex::split(command);

  if let None = args {
    return Err(Error::Unknown { code: 1, message: "malformed command, cannot be parsed".to_string() })
  }

  let args = args.unwrap();
  if args.is_empty() {
    return Err(Error::Unknown { code: 1, message: "Command was empty".to_string() });
  }

  println!("{args:?}");
  Ok(args)
}

pub fn create_command(argv: Vec<String>, no_color: bool) -> Result<Command, Error> {
  let mut args = argv.iter();
  let arg0 = args.next()
    .ok_or_else(|| Error::Unknown { code: 1, message: "Expected command binary".to_string() })?;

  let mut cmd = Command::new(arg0);
  cmd.args(args);

  if no_color {
    cmd.env("NO_COLOR", "1");
  }

  Ok(cmd)
}

pub fn execute_command(command: impl Into<String>, no_color: bool) -> Result<(), Error> {
  let args = parse_args(command)?;

  let exit_code = create_command(args, no_color)?
    .stdin(Stdio::inherit())
    .spawn().expect("Failed to execute command")
    .wait().expect("Failed to wait for command");

  if let Some(code) = exit_code.code() {
    if !code.eq(&0) {
      return Err(Error::CommandFailed { code });
    }
  }

  Ok(())
}

pub fn catch_output(command: impl Into<String>, no_color: bool) -> Result<String, Error> {
  let args = parse_args(command)?;

  let output = create_command(args, no_color)?
    .output()
    .map_err(|e| Error::Unknown { code: 1, message: format!("failed to capture output: '{}'", e) })?;

  if !output.status.success() {
    let code = output.status.code().unwrap_or(-1);
    return Err(Error::CommandFailed { code });
  }

  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
