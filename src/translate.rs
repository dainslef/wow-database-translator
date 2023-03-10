use crate::{common::*, data::*};
use log::{debug, info};
use sqlx::{
  mysql::{MySqlArguments, MySqlRow},
  query::Query,
  MySql, Row,
};
use tokio::{task::JoinSet, try_join};

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

  let origin_count = data_count::<T>(origin_language).await?;
  let target_count = data_count::<T>(!origin_language).await?;
  if origin_count == target_count {
    info!("Table {database}.{table} has already been translated (total count: {origin_count}), skipping ...");
    return Ok(());
  }
  info!("Translating table {database}.{table} (total count: {origin_count}) ...");

  let mut translate_rows_count = 0;
  for i in (0..origin_count).step_by(COMMAND_LINE.batch_size) {
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
    info!("{database}.{table} Progress: {i}/{origin_count}");
    debug!("{database}.{table} Rows affected: {insert_results:?}");
  }

  info!("Translate table {database}.{table} finished (translate rows count: {translate_rows_count}/{origin_count}) ...");
  Ok(())
}

macro_rules! translate_tables {
  ($language: ident, $($data_type: ty),*) => {
    $(translate_table::<$data_type>($language).await?;)*
  };
  (+ $language: ident, $($data_type: ty),*) => {
    try_join!($(translate_table::<$data_type>($language),)*)?
  };
}

/// Table translate logic.
pub async fn translate_tables(origin_language: Language) -> anyhow::Result<()> {
  info!("Run table translate ...");

  if COMMAND_LINE.r#async {
    translate_tables!(
      + origin_language,
      AchievementRewardLocale,
      BroadcastTextLocale,
      CreatureTemplateLocale,
      CreatureTextLocale,
      GameobjectTemplateLocale,
      GossipMenuOptionLocale,
      ItemSetNamesLocale,
      ItemTemplateLocale,
      NpcTextLocale,
      PageTextLocale,
      PointsOfInterestLocale,
      QuestGreetingLocale,
      QuestOfferRewardLocale,
      QuestRequestItemsLocale,
      QuestTemplateLocale
    );
    // Macro expanded:
    // try_join!(
    //   translate_table::<AchievementRewardLocale>(origin_language),
    //   translate_table::<BroadcastTextLocale>(origin_language),
    //   ...
    // )?;
  } else {
    translate_tables!(
      origin_language,
      AchievementRewardLocale,
      BroadcastTextLocale,
      CreatureTemplateLocale,
      CreatureTextLocale,
      GameobjectTemplateLocale,
      GossipMenuOptionLocale,
      ItemSetNamesLocale,
      ItemTemplateLocale,
      NpcTextLocale,
      PageTextLocale,
      PointsOfInterestLocale,
      QuestGreetingLocale,
      QuestOfferRewardLocale,
      QuestRequestItemsLocale,
      QuestTemplateLocale
    );
    // Macro expanded:
    // translate_table::<AchievementRewardLocale>(origin_language).await?;
    // translate_table::<BroadcastTextLocale>(origin_language).await?;
    // ...
  }

  Ok(())
}

async fn check_translation<
  T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic,
>() -> anyhow::Result<(bool, &'static str)> {
  let TranslateTarget {
    database, table, ..
  } = T::TARGET;

  let taiwanese_count = data_count::<T>(Language::Taiwanese).await?;
  let chinese_count = data_count::<T>(Language::Chinese).await?;
  info!(
    "Table {database}.{table} is equal: {} (taiwanese count: {taiwanese_count}, chinese count: {chinese_count}) ... ",
    taiwanese_count == chinese_count
  );

  Ok((taiwanese_count == chinese_count, table))
}

macro_rules! check_translations {
  (+ $join_set: ident, $($data_type: ty),*) => {
    $($join_set.spawn(check_translation::<$data_type>());)*
  };
  ($($data_type: ty),*) => {
    vec![$(check_translation::<$data_type>().await?,)*]
  };
}

/// Table translation check logic.
pub async fn check_translations() -> anyhow::Result<()> {
  info!("Check table translations ...");

  let task_results = if COMMAND_LINE.r#async {
    let mut join_set = JoinSet::new();

    check_translations!(
      + join_set,
      AchievementRewardLocale,
      BroadcastTextLocale,
      CreatureTemplateLocale,
      CreatureTextLocale,
      GameobjectTemplateLocale,
      GossipMenuOptionLocale,
      ItemSetNamesLocale,
      ItemTemplateLocale,
      NpcTextLocale,
      PageTextLocale,
      PointsOfInterestLocale,
      QuestGreetingLocale,
      QuestOfferRewardLocale,
      QuestRequestItemsLocale,
      QuestTemplateLocale
    );

    let mut results = vec![];
    while let Some(result) = join_set.join_next().await {
      results.push(result??);
    }
    results
  } else {
    check_translations!(
      AchievementRewardLocale,
      BroadcastTextLocale,
      CreatureTemplateLocale,
      CreatureTextLocale,
      GameobjectTemplateLocale,
      GossipMenuOptionLocale,
      ItemSetNamesLocale,
      ItemTemplateLocale,
      NpcTextLocale,
      PageTextLocale,
      PointsOfInterestLocale,
      QuestGreetingLocale,
      QuestOfferRewardLocale,
      QuestRequestItemsLocale,
      QuestTemplateLocale
    )
  };

  let not_equal_tables: Vec<_> = task_results
    .into_iter()
    .filter(|(v, _)| *v == false)
    .map(|(_, t)| t)
    .collect();
  if not_equal_tables.is_empty() {
    info!("All tables' translation count are equal.");
  } else {
    info!("Some tables' translation count aren't equal: {not_equal_tables:?}.");
  }

  Ok(())
}

#[tokio::test]
async fn query_test() -> anyhow::Result<()> {
  use crate::{data::QuestTemplateLocale, ConvertText, Language};
  use opencc_rust::{DefaultConfig, OpenCC};
  use sqlx::MySqlPool;

  let pool = MySqlPool::connect("mysql://root:password@localhost/acore_world").await?;
  let results = sqlx::query_as::<MySql, QuestTemplateLocale>(
    "SELECT * FROM quest_template_locale WHERE locale = ? OR locale = ? LIMIT ?, ?",
  )
  .bind(Language::Taiwanese.to_string())
  .bind(Language::Chinese.to_string())
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
