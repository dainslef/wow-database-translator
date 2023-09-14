use crate::{
  common::*,
  data::{azeroth_core, mangos},
};
use log::{debug, info};
use opencc_rust::OpenCC;
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

async fn translate<T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic>(
  origin_language: Language,
  origin_count: i64,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database,
    table,
    locale_column,
  } = T::TARGET;

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

async fn translate_table<T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic>(
) -> anyhow::Result<()> {
  let (is_equal, _, (taiwanese_count, chinese_count)) = check_translation::<T>().await?;
  if !is_equal {
    translate::<T>(Language::Taiwanese, taiwanese_count).await?;
    translate::<T>(Language::Chinese, chinese_count).await?;
  }
  Ok(())
}

macro_rules! translate_tables {
  ($($data_type: ty),*) => {
    $(translate_table::<$data_type>().await?;)*
  };
  (+$($data_type: ty),*) => {
    try_join!($(translate_table::<$data_type>(),)*)?
  };
}

/// Table translate logic.
pub async fn translate_tables() -> anyhow::Result<()> {
  info!("Run table translate ...");

  if COMMAND_LINE.r#async {
    translate_tables!(
      +
      azeroth_core::AchievementRewardLocale,
      azeroth_core::BroadcastTextLocale,
      azeroth_core::CreatureTemplateLocale,
      azeroth_core::CreatureTextLocale,
      azeroth_core::GameobjectTemplateLocale,
      azeroth_core::GossipMenuOptionLocale,
      azeroth_core::ItemSetNamesLocale,
      azeroth_core::ItemTemplateLocale,
      azeroth_core::NpcTextLocale,
      azeroth_core::PageTextLocale,
      azeroth_core::PointsOfInterestLocale,
      azeroth_core::QuestGreetingLocale,
      azeroth_core::QuestOfferRewardLocale,
      azeroth_core::QuestRequestItemsLocale,
      azeroth_core::QuestTemplateLocale
    );
    // Macro expanded:
    // try_join!(
    //   translate_table::<AchievementRewardLocale>(),
    //   translate_table::<BroadcastTextLocale>(),
    //   ...
    // )?;
  } else {
    translate_tables!(
      azeroth_core::AchievementRewardLocale,
      azeroth_core::BroadcastTextLocale,
      azeroth_core::CreatureTemplateLocale,
      azeroth_core::CreatureTextLocale,
      azeroth_core::GameobjectTemplateLocale,
      azeroth_core::GossipMenuOptionLocale,
      azeroth_core::ItemSetNamesLocale,
      azeroth_core::ItemTemplateLocale,
      azeroth_core::NpcTextLocale,
      azeroth_core::PageTextLocale,
      azeroth_core::PointsOfInterestLocale,
      azeroth_core::QuestGreetingLocale,
      azeroth_core::QuestOfferRewardLocale,
      azeroth_core::QuestRequestItemsLocale,
      azeroth_core::QuestTemplateLocale
    );
    // Macro expanded:
    // translate_table::<AchievementRewardLocale>().await?;
    // translate_table::<BroadcastTextLocale>().await?;
    // ...
  }

  Ok(())
}

async fn check_translation<
  T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + TranslateLogic,
>() -> anyhow::Result<(bool, &'static str, (i64, i64))> {
  let TranslateTarget {
    database, table, ..
  } = T::TARGET;

  let taiwanese_count = data_count::<T>(Language::Taiwanese).await?;
  let chinese_count = data_count::<T>(Language::Chinese).await?;
  info!(
    "Table {database}.{table} is equal: {} (taiwanese count: {taiwanese_count}, chinese count: {chinese_count}) ... ",
    taiwanese_count == chinese_count
  );

  Ok((
    taiwanese_count == chinese_count,
    table,
    (taiwanese_count, chinese_count),
  ))
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
      azeroth_core::AchievementRewardLocale,
      azeroth_core::BroadcastTextLocale,
      azeroth_core::CreatureTemplateLocale,
      azeroth_core::CreatureTextLocale,
      azeroth_core::GameobjectTemplateLocale,
      azeroth_core::GossipMenuOptionLocale,
      azeroth_core::ItemSetNamesLocale,
      azeroth_core::ItemTemplateLocale,
      azeroth_core::NpcTextLocale,
      azeroth_core::PageTextLocale,
      azeroth_core::PointsOfInterestLocale,
      azeroth_core::QuestGreetingLocale,
      azeroth_core::QuestOfferRewardLocale,
      azeroth_core::QuestRequestItemsLocale,
      azeroth_core::QuestTemplateLocale
    );

    let mut results = vec![];
    while let Some(result) = join_set.join_next().await {
      results.push(result??);
    }
    results
  } else {
    check_translations!(
      azeroth_core::AchievementRewardLocale,
      azeroth_core::BroadcastTextLocale,
      azeroth_core::CreatureTemplateLocale,
      azeroth_core::CreatureTextLocale,
      azeroth_core::GameobjectTemplateLocale,
      azeroth_core::GossipMenuOptionLocale,
      azeroth_core::ItemSetNamesLocale,
      azeroth_core::ItemTemplateLocale,
      azeroth_core::NpcTextLocale,
      azeroth_core::PageTextLocale,
      azeroth_core::PointsOfInterestLocale,
      azeroth_core::QuestGreetingLocale,
      azeroth_core::QuestOfferRewardLocale,
      azeroth_core::QuestRequestItemsLocale,
      azeroth_core::QuestTemplateLocale
    )
  };

  let not_equal_tables: Vec<_> = task_results
    .into_iter()
    .filter(|(v, _, _)| *v == false)
    .map(|(_, t, _)| t)
    .collect();
  if not_equal_tables.is_empty() {
    info!("All tables' translation count are equal.");
  } else {
    info!("Some tables' translation count aren't equal: {not_equal_tables:?}.");
  }

  Ok(())
}

async fn data_count_mangos(
  translate_target: &TranslateTarget,
  origin_language: Language,
) -> anyhow::Result<i64> {
  let TranslateTarget {
    database,
    table,
    locale_column,
  } = translate_target;

  let origin_locale_colume = mangos::column_name((*locale_column).into(), origin_language);
  let target_locale_column = mangos::column_name((*locale_column).into(), !origin_language);

  let count: i64 = sqlx::query::<MySql>(&format!(
    "SELECT count(*) FROM {database}.{table} WHERE {origin_locale_colume} IS NOT NULL AND {origin_locale_colume} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '')"
  ))
  .fetch_one(&*POOL)
  .await?
  .get("count(*)");

  Ok(count)
}

async fn translate_mangos(
  translate_target: &TranslateTarget,
  origin_language: Language,
  origin_count: i64,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database,
    table,
    locale_column,
  } = translate_target;

  let origin_locale_colume = mangos::column_name((*locale_column).into(), origin_language);
  let target_locale_column = mangos::column_name((*locale_column).into(), !origin_language);

  info!("Translating table {database}.{table} (total count: {origin_count}) ...");

  let mut translate_rows_count = 0;
  for i in (0..origin_count).step_by(COMMAND_LINE.batch_size) {
    let results = sqlx::query::<MySql>(&format!(
      "SELECT entry,{origin_locale_colume},{target_locale_column} FROM {database}.{table} WHERE {origin_locale_colume} IS NOT NULL and {origin_locale_colume} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '') LIMIT {i}, {}",
      COMMAND_LINE.batch_size
    ))
    .fetch_all(&*POOL)
    .await?;

    let mut insert_results = vec![];
    for v in results {
      let entry: u32 = v.get("entry");
      let origin_text: String = v.get(&*origin_locale_colume);
      let opencc: &OpenCC = origin_language.into();
      let target_text = opencc.convert(origin_text);

      // Execute the insert SQL.
      let rows_affected = sqlx::query(&format!(
        "UPDATE {database}.{table} SET {target_locale_column} = ? WHERE entry = {entry}"
      ))
      .bind(target_text)
      .execute(&*POOL)
      .await?
      .rows_affected();
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

pub async fn translate_tables_mangos(database: &'static str) -> anyhow::Result<()> {
  let targets: Vec<TranslateTarget> = vec![
    ("locales_creature", "name"),
    ("locales_creature", "subname"),
    ("locales_gameobject", "name"),
    // ("locales_gossip_menu_option", "option_text"),
    // ("locales_gossip_menu_option", "box_text"),
    ("locales_item", "name"),
    ("locales_item", "description"),
    ("locales_quest", "Title"),
    ("locales_quest", "Details"),
    ("locales_quest", "Objectives"),
    ("locales_quest", "OfferRewardText"),
    ("locales_quest", "EndText"),
    ("locales_quest", "ObjectiveText1"),
    ("locales_quest", "ObjectiveText2"),
    ("locales_quest", "ObjectiveText3"),
    ("locales_quest", "ObjectiveText4"),
    ("locales_points_of_interest", "icon_name"),
    ("locales_page_text", "Text"),
    ("locales_npc_text", "Text0_0"),
    ("locales_npc_text", "Text0_1"),
    ("locales_npc_text", "Text1_0"),
    ("locales_npc_text", "Text1_1"),
    ("locales_npc_text", "Text2_0"),
    ("locales_npc_text", "Text2_1"),
    ("locales_npc_text", "Text3_0"),
    ("locales_npc_text", "Text3_1"),
    ("locales_npc_text", "Text4_0"),
    ("locales_npc_text", "Text4_1"),
    ("locales_npc_text", "Text5_0"),
    ("locales_npc_text", "Text5_1"),
    ("locales_npc_text", "Text6_0"),
    ("locales_npc_text", "Text6_1"),
    ("locales_npc_text", "Text7_0"),
    ("locales_npc_text", "Text7_1"),
  ]
  .into_iter()
  .map(|(table, locale_column)| TranslateTarget {
    database,
    table,
    locale_column,
  })
  .collect();

  async fn translate(
    origin_language: Language,
    translate_target: &TranslateTarget,
  ) -> anyhow::Result<()> {
    let origin_count = data_count_mangos(translate_target, origin_language).await?;
    if origin_count > 0 {
      translate_mangos(translate_target, origin_language, origin_count).await?;
    }
    Ok(())
  }

  for translate_target in targets {
    translate(Language::Chinese, &translate_target).await?;
    translate(Language::Taiwanese, &translate_target).await?;
  }

  Ok(())
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
