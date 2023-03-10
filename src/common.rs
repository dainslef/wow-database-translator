use clap::Parser;
use log::{debug, LevelFilter};
use once_cell::sync::Lazy;
use opencc_rust::{DefaultConfig, OpenCC};
use sqlx::{mysql::MySqlConnectOptions, ConnectOptions, MySqlPool};
use std::{future::Future, ops::Not, str::FromStr};
use strum::ParseError;
use strum_macros::Display;

/// Run aysnc method as sync (block thread and wait result).
pub fn block_async<F>(f: F) -> F::Output
where
  F: Future + Send + 'static,
  F::Output: Send,
{
  // Get current tokio runtime handle.
  let handle = tokio::runtime::Handle::current();
  // Spawn a new thread to block.
  std::thread::spawn(move || {
    // Handle can't block on tokio main thread. Error message:
    // "thread 'main' panicked at 'Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks."
    handle.block_on(f)
  })
  .join()
  .expect("Wait thread spawn result failed!")
}

pub fn init_logger() {
  // The env_logger's default log level is Error, need to change it manually.
  env_logger::builder()
    .filter_level(COMMAND_LINE.log)
    .try_init()
    .expect("Init env_logger failed!");
  debug!("Command line args: {COMMAND_LINE:?}");
}

/// Define the language types.
#[derive(Clone, Copy, Display, Debug, strum_macros::EnumString, sqlx::Type, clap::ValueEnum)]
pub enum Language {
  #[strum(serialize = "zhCN")]
  Chinese,
  #[strum(serialize = "zhTW")]
  Taiwanese,
}

impl From<Language> for &OpenCC {
  fn from(value: Language) -> Self {
    match value {
      Language::Taiwanese => &OPECC_TW2SP,
      Language::Chinese => &OPECC_S2TWP,
    }
  }
}

impl Not for Language {
  type Output = Self;
  fn not(self) -> Self {
    match self {
      Self::Chinese => Self::Taiwanese,
      Self::Taiwanese => Self::Chinese,
    }
  }
}

/// Implement TryFrom can transform supported type to custom type in SQLx.
impl TryFrom<String> for Language {
  type Error = ParseError;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::from_str(&value)
  }
}

pub trait ConvertText {
  fn convert_text(&self, text: &Option<String>) -> String;

  /// Try to convert option string text.
  fn convert_impl<'a>(opencc: impl Into<&'a OpenCC>, text: &Option<String>) -> String {
    opencc.into().convert(text.as_ref().unwrap_or(&"".into()))
  }
}

impl ConvertText for OpenCC {
  fn convert_text(&self, text: &Option<String>) -> String {
    Self::convert_impl(self, text)
  }
}

impl ConvertText for Language {
  fn convert_text(&self, text: &Option<String>) -> String {
    Self::convert_impl(*self, text)
  }
}

#[derive(Parser, Debug)]
#[command(
  version,
  about = "WOW database translator",
  long_about = "WOW database translator\nIt's an application to translate WOW locale database tables between zhTW and zhCN."
)]
pub struct CommandLine {
  /// Set the database target address
  #[arg(long, default_value = "127.0.0.1")]
  pub host: String,
  /// Set the database target port
  #[arg(long, default_value = "3306")]
  pub port: u16,
  /// Set the database login username
  #[arg(short, long, default_value = "root")]
  pub username: String,
  /// Set the database login password
  #[arg(short, long, default_value = "password")]
  pub password: String,
  /// Set the default database
  #[arg(short, long)]
  pub database: Option<String>,
  /// Set the data batch size
  #[arg(short, long, default_value = "1000")]
  pub batch_size: usize,
  /// Enable async execute
  #[arg(short, long)]
  pub r#async: bool,
  /// Run database translation check
  #[arg(short, long)]
  pub check: bool,
  /// Execute database translate
  #[arg(short, long, value_enum, name = "ORIGIN_LANGUAGE")]
  pub translate: Option<Language>,
  /// Set the log level filter
  #[arg(short, long, default_value = "info")]
  pub log: LevelFilter,
}

/// OpenCC configs.
static OPECC_S2TWP: Lazy<OpenCC> =
  Lazy::new(|| OpenCC::new(DefaultConfig::S2TWP).expect("Init OpenCC error!"));
static OPECC_TW2SP: Lazy<OpenCC> =
  Lazy::new(|| OpenCC::new(DefaultConfig::TW2SP).expect("Init OpenCC error!"));

/// Global lazy instances.
pub static COMMAND_LINE: Lazy<CommandLine> = Lazy::new(|| CommandLine::parse());
pub static POOL: Lazy<MySqlPool> = Lazy::new(|| {
  block_async({
    let mut options = MySqlConnectOptions::new()
      .host(&COMMAND_LINE.host)
      .port(COMMAND_LINE.port)
      .username(&COMMAND_LINE.username)
      .password(&COMMAND_LINE.password);
    if let Some(database) = &COMMAND_LINE.database {
      options = options.database(database);
    }
    options.log_statements(LevelFilter::Debug);
    MySqlPool::connect_with(options)
  })
  .expect("Init MySQL connection error!")
});
