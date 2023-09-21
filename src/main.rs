mod common;
mod data;
mod translate;

use clap::CommandFactory;
use common::*;
use translate::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_logger();

  if let Some(translate_type) = &COMMAND_LINE.translate {
    match translate_type {
      ServerType::AzerothCore => azeroth_core::translate_tables().await?,
      v => mangos::translate_tables(v).await?,
    }
  } else if COMMAND_LINE.check {
    azeroth_core::check_translations().await?;
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
