use super::{Operation, Operations};
use crate::{CLI_NAME, Cli, error::Error, options::Options};

pub struct Help;

impl Operation for Help {
  fn operate(_cli: &Cli) -> Result<(), Error> {
    println!("usage:  {CLI_NAME} <operation> [...]");
    println!("operations:");
    Operations::print_help();
    println!("");
    println!("options without operations:");
    println!("  {}", Options::Clean);
    println!("\nuse '{CLI_NAME} {}' with an operation for available options", Operations::Help);

    Ok(())
  }
}
