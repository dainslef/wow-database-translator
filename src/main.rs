mod common;
mod data;
mod translate;

use common::*;
use translate::*;

use clap::CommandFactory;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_logger();

  if let Some(language) = COMMAND_LINE.translate {
    translate_tables(language).await?;
  } else if COMMAND_LINE.check {
    check_translation().await?;
  } else {
    // Print help message when there is no action command input.
    CommandLine::command().print_long_help()?;
  }

  Ok(())
}

#[test]
fn env_logger_test() {
  init_logger();
  log::info!("Fuck CCP!");
}
