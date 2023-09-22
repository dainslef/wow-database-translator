mod common;
mod data;
mod translate;

use clap::CommandFactory;
use common::*;
use translate::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_logger();

  if let Some(ServerType::AzerothCore) = &COMMAND_LINE.translate {
    azeroth_core::translate_tables().await?;
  } else if let Some(v) = &COMMAND_LINE.translate {
    mangos::translate_tables(v).await?;
  } else if let Some(ServerType::AzerothCore) = &COMMAND_LINE.check {
    azeroth_core::check_translations().await?;
  } else if let Some(v) = &COMMAND_LINE.check {
    mangos::check_translations(v).await?;
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
