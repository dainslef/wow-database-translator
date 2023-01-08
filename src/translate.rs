use crate::{common::*, data::*};
use log::{debug, info};
use sqlx::{
  mysql::{MySqlArguments, MySqlRow},
  query::Query,
  MySql, Row,
};

pub struct TranslateTarget {
  pub database: &'static str,
  pub table: &'static str,
  pub locale_column: &'static str,
}

impl TranslateTarget {
  pub const fn new(
    database: &'static str,
    table: &'static str,
    locale_column: &'static str,
  ) -> Self {
    TranslateTarget {
      database,
      table,
      locale_column,
    }
  }
}

pub trait TranslateLogic {
  const TARGET: TranslateTarget;
  const SQL: &'static str;
  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments>;
}

async fn translate<T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic>(
  origin_language: Language,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database,
    table,
    locale_column,
  } = T::TARGET;

  let count: i64 = sqlx::query::<MySql>(&format!(
    "SELECT count(*) FROM {database}.{table} WHERE {locale_column} = '{origin_language}'"
  ))
  .fetch_one(&*POOL)
  .await?
  .get("count(*)");
  info!("Translate table {database}.{table} (total count: {count}) ... ");

  let mut translate_rows_count = 0;
  for i in (0..count).step_by(COMMAND_LINE.batch_size) {
    let results = sqlx::query_as::<MySql, T>(&format!(
      "SELECT * FROM {database}.{table} WHERE {locale_column} = '{origin_language}' LIMIT {i}, {}",
      COMMAND_LINE.batch_size
    ))
    .fetch_all(&*POOL)
    .await?;

    let mut insert_results = vec![];
    for v in results {
      // Execute the insert SQL.
      let rows_affected = v.bind_query().execute(&*POOL).await?.rows_affected();
      insert_results.push(rows_affected);
      translate_rows_count += rows_affected;
    }

    // Log the execute progress and row affects.
    info!("{database}.{table} Progress: {i}/{count}");
    debug!("{database}.{table} Rows affected: {insert_results:?}");
  }

  info!("Translate table {database}.{table} finished (translate rows count: {translate_rows_count}/{count}) ...");
  Ok(())
}

/// Table translate logic.
pub async fn translate_tables(origin_language: Language) -> anyhow::Result<()> {
  info!("Run table translate ...");

  translate::<AchievementRewardLocale>(origin_language).await?;
  translate::<BroadcastTextLocale>(origin_language).await?;
  translate::<CreatureTemplateLocale>(origin_language).await?;
  translate::<CreatureTextLocale>(origin_language).await?;
  translate::<GameobjectTemplateLocale>(origin_language).await?;
  translate::<GossipMenuOptionLocale>(origin_language).await?;
  translate::<ItemSetNamesLocale>(origin_language).await?;
  translate::<ItemTemplateLocale>(origin_language).await?;
  translate::<NpcTextLocale>(origin_language).await?;
  translate::<PageTextLocale>(origin_language).await?;
  translate::<PointsOfInterestLocale>(origin_language).await?;
  translate::<QuestGreetingLocale>(origin_language).await?;
  translate::<QuestOfferRewardLocale>(origin_language).await?;
  translate::<QuestRequestItemsLocale>(origin_language).await?;
  translate::<QuestTemplateLocale>(origin_language).await?;

  Ok(())
}

/// Table translation check logic.
pub async fn check_translation() -> anyhow::Result<()> {
  info!("Check table translations ...");
  Ok(())
}

#[tokio::test]
async fn query_test() -> anyhow::Result<()> {
  use crate::data::QuestTemplateLocale;
  use opencc_rust::{DefaultConfig, OpenCC};
  use sqlx::MySqlPool;

  let pool = MySqlPool::connect("mysql://root:password@localhost/acore_world").await?;
  let results = sqlx::query_as::<MySql, QuestTemplateLocale>(
    "SELECT * FROM quest_template_locale WHERE locale = ? OR locale = ? LIMIT ?, ?",
  )
  .bind("zhCN")
  .bind("zhTW")
  .bind(0)
  .bind(10)
  .fetch_all(&pool)
  .await?;
  let opencc = OpenCC::new(DefaultConfig::S2TWP).expect("Init OpenCC failed!");

  println!("Data count: {}", results.len());
  for result in results {
    println!(
      "Data: {result:?}\nOrigin: {}\nTranslate: {}",
      &result.details,
      opencc.convert(&result.details)
    );
  }

  Ok(())
}
