use std::collections::HashMap;

use log::*;
use opencc_rust::OpenCC;
use sqlx::{MySql, Row};

use crate::{
  common::{Language, ServerType, COMMAND_LINE, POOL},
  data::mangos::{self, get_translate_targets},
};

use super::TranslateTarget;

async fn data_count(
  translate_target: &TranslateTarget,
  origin_language: Language,
) -> anyhow::Result<Option<Vec<(String, String, i64)>>> {
  let TranslateTarget {
    database,
    table,
    locale_columns,
  } = translate_target;
  let mut counts = vec![];
  let mut need_translate = false;

  for locale_column in locale_columns {
    let origin_locale_column = mangos::column_name(locale_column, origin_language);
    let target_locale_column = mangos::column_name(locale_column, !origin_language);

    let count: i64 = sqlx::query::<MySql>(&format!(
      "SELECT count(*) FROM {database}.{table} WHERE {origin_locale_column} IS NOT NULL AND {origin_locale_column} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '')"))
      .fetch_one(&*POOL)
      .await?
      .get("count(*)");

    // Only add tables which need to be translated.
    if count > 0 {
      need_translate = true;
      counts.push((origin_locale_column, target_locale_column, count));
    }
  }

  Ok(if need_translate { Some(counts) } else { None })
}

async fn check_translation(
  translate_target: &TranslateTarget,
) -> anyhow::Result<HashMap<Language, Vec<(String, String, i64)>>> {
  let TranslateTarget {
    database, table, ..
  } = translate_target;
  let mut need_translate_content = HashMap::new();

  let taiwanese_count = data_count(translate_target, Language::Taiwanese).await?;
  let chinese_count = data_count(translate_target, Language::Chinese).await?;

  let need_translate = !need_translate_content.is_empty();
  info!("Table {database}.{table} has untranslate contents: {need_translate} (taiwanese count: {taiwanese_count:?}, chinese count: {chinese_count:?}) ... ");

  if let Some(taiwanese_count) = taiwanese_count {
    need_translate_content.insert(Language::Taiwanese, taiwanese_count);
  }
  if let Some(chinese_count) = chinese_count {
    need_translate_content.insert(Language::Chinese, chinese_count);
  }

  Ok(need_translate_content)
}

/// Table translation check logic.
pub async fn check_translations(server_type: &ServerType) -> anyhow::Result<()> {
  info!("Check table translations ...");
  let mut need_translate_tables = vec![];
  let translate_targets = get_translate_targets(server_type);

  for translate_target in &translate_targets {
    let need_translate_content = check_translation(translate_target).await?;
    if !need_translate_content.is_empty() {
      need_translate_tables.push(&translate_target.table);
    }
  }

  if need_translate_tables.is_empty() {
    info!("All tables' translation count are equal.");
  } else {
    info!("Some tables' translation count aren't equal: {need_translate_tables:?}.");
  }

  Ok(())
}

async fn translate_column(
  translate_target: &TranslateTarget,
  origin_language: Language,
  origin_count: i64,
  origin_locale_column: &String,
  target_locale_column: &String,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database, table, ..
  } = translate_target;
  info!(
    "Translating table {database}.{table} from {origin_language} (total count: {origin_count}) ..."
  );

  let (mut translate_rows_count, batch_size) = (0, COMMAND_LINE.batch_size);
  for i in (0..origin_count).step_by(COMMAND_LINE.batch_size) {
    let results = sqlx::query::<MySql>(&format!(
      "SELECT entry,{origin_locale_column},{target_locale_column} FROM {database}.{table} WHERE {origin_locale_column} IS NOT NULL AND {origin_locale_column} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '') LIMIT {batch_size}"
    ))
    .fetch_all(&*POOL)
    .await?;

    let mut insert_results = vec![];
    for v in results {
      let entry: u32 = v.get("entry");
      let origin_text: String = v.get(origin_locale_column.as_str());
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

  info!("Translate table {database}.{table} from {origin_locale_column} to {target_locale_column} finished (translate rows count: {translate_rows_count}/{origin_count}) ...");

  Ok(())
}

pub async fn translate_tables(server_type: &ServerType) -> anyhow::Result<()> {
  for translate_target in &mangos::get_translate_targets(server_type) {
    for (language, columns) in check_translation(translate_target).await? {
      for (origin_locale_column, target_locale_column, count) in columns {
        translate_column(
          translate_target,
          language,
          count,
          &origin_locale_column,
          &target_locale_column,
        )
        .await?;
      }
    }
  }
  Ok(())
}
