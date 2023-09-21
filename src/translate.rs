pub mod azeroth_core;
pub mod mangos;

use once_cell::sync::Lazy;
use sqlx::{MySql, QueryBuilder};

pub struct TranslateTarget {
  pub database: String,
  pub table: String,
  pub locale_columns: Vec<String>,
}

impl TranslateTarget {
  pub fn new(database: impl ToString, table: impl ToString, locale_column: impl ToString) -> Self {
    TranslateTarget {
      database: database.to_string(),
      table: table.to_string(),
      locale_columns: vec![locale_column.to_string()],
    }
  }

  pub fn multi_columns(
    database: impl ToString,
    table: impl ToString,
    locale_columns: Vec<String>,
  ) -> Self {
    TranslateTarget {
      database: database.to_string(),
      table: table.to_string(),
      locale_columns,
    }
  }
}

pub trait TranslateLogic {
  const TARGET: Lazy<TranslateTarget>;
  fn build_query(&self) -> QueryBuilder<'static, MySql>;
}

#[tokio::test]
async fn query_test() -> anyhow::Result<()> {
  use crate::{data::azeroth_core::QuestTemplateLocale, ConvertText, Language};
  use opencc_rust::{DefaultConfig, OpenCC};
  use sqlx::MySqlPool;

  let pool = MySqlPool::connect("mysql://root:password@localhost/acore_world").await?;
  let results = sqlx::query_as::<MySql, QuestTemplateLocale>(
    "SELECT * FROM quest_template_locale WHERE locale = ? OR locale = ? LIMIT ?, ?",
  )
  .bind(Language::Taiwanese)
  .bind(Language::Chinese)
  .bind(0)
  .bind(10)
  .fetch_all(&pool)
  .await?;
  let opencc = OpenCC::new(DefaultConfig::S2TWP).expect("Init OpenCC failed!");

  println!("Data count: {}", results.len());
  for result in results {
    println!(
      "Data: {result:?}\nOrigin: {:?}\nTranslate: {}",
      &result.details,
      opencc.convert_text(&result.details)
    );
  }

  Ok(())
}
