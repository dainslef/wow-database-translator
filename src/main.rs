mod common;
mod data;
mod translate;

use clap::CommandFactory;
use common::*;
use translate::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_logger();

  if COMMAND_LINE.translate {
    match &COMMAND_LINE.server_type {
      ServerType::AzerothCore => translate_tables().await?,
      v => translate_tables_mangos(v.database()).await?,
    }
  } else if COMMAND_LINE.check {
    check_translations().await?;
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
