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

async fn data_count<T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic>(
  origin_language: Language,
) -> anyhow::Result<i64> {
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

  Ok(count)
}

async fn translate_table<T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic>(
  origin_language: Language,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database,
    table,
    locale_column,
  } = T::TARGET;

  let count: i64 = data_count::<T>(origin_language).await?;
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

  translate_table::<AchievementRewardLocale>(origin_language).await?;
  translate_table::<BroadcastTextLocale>(origin_language).await?;
  translate_table::<CreatureTemplateLocale>(origin_language).await?;
  translate_table::<CreatureTextLocale>(origin_language).await?;
  translate_table::<GameobjectTemplateLocale>(origin_language).await?;
  translate_table::<GossipMenuOptionLocale>(origin_language).await?;
  translate_table::<ItemSetNamesLocale>(origin_language).await?;
  translate_table::<ItemTemplateLocale>(origin_language).await?;
  translate_table::<NpcTextLocale>(origin_language).await?;
  translate_table::<PageTextLocale>(origin_language).await?;
  translate_table::<PointsOfInterestLocale>(origin_language).await?;
  translate_table::<QuestGreetingLocale>(origin_language).await?;
  translate_table::<QuestOfferRewardLocale>(origin_language).await?;
  translate_table::<QuestRequestItemsLocale>(origin_language).await?;
  translate_table::<QuestTemplateLocale>(origin_language).await?;

  Ok(())
}

async fn check_translation<
  T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic,
>() -> anyhow::Result<()> {
  let TranslateTarget {
    database, table, ..
  } = T::TARGET;

  let taiwanese_count: i64 = data_count::<T>(Language::Taiwanese).await?;
  let chinese_count: i64 = data_count::<T>(Language::Chinese).await?;
  info!(
    "Table {database}.{table} is equal: {} (taiwanese count: {taiwanese_count}, chinese count: {chinese_count}) ... ",
    taiwanese_count == chinese_count
  );

  Ok(())
}

/// Table translation check logic.
pub async fn check_translations() -> anyhow::Result<()> {
  info!("Check table translations ...");

  check_translation::<AchievementRewardLocale>().await?;
  check_translation::<BroadcastTextLocale>().await?;
  check_translation::<CreatureTemplateLocale>().await?;
  check_translation::<CreatureTextLocale>().await?;
  check_translation::<GameobjectTemplateLocale>().await?;
  check_translation::<GossipMenuOptionLocale>().await?;
  check_translation::<ItemSetNamesLocale>().await?;
  check_translation::<ItemTemplateLocale>().await?;
  check_translation::<NpcTextLocale>().await?;
  check_translation::<PageTextLocale>().await?;
  check_translation::<PointsOfInterestLocale>().await?;
  check_translation::<QuestGreetingLocale>().await?;
  check_translation::<QuestOfferRewardLocale>().await?;
  check_translation::<QuestRequestItemsLocale>().await?;
  check_translation::<QuestTemplateLocale>().await?;

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
