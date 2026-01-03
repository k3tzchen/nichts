use std::process::{ Command, Stdio };

use crate::error::Error;

pub fn prepare_cmd(argv: Vec<&str>, no_color: bool) -> Command {
  let mut args = argv.iter();
  let arg0 = args.next().expect("Expected nix command");
  let mut cmd = Command::new(arg0);

  cmd.args(args);

  if no_color {
    cmd.env("NO_COLOR", "1");
  }

  cmd
}

pub fn exec_cmd(command: impl Into<String>, no_color: bool) -> Result<(), Error> {
  let command = command.into();
  let command = command.as_str();
  let args = command.split_ascii_whitespace();
  let args = args.collect::<Vec<&str>>();

  let exit_code = prepare_cmd(args, no_color)
    .stdin(Stdio::inherit())
    .spawn()
    .expect("Failed to execute command")
    .wait()
    .expect("Failed to wait for command");

  if let Some(code) = exit_code.code() {
    if !code.eq(&0) {
      return Err(Error::CommandFailed { code });
    }
  }

  return Ok(());
}
