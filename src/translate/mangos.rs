use log::*;
use opencc_rust::OpenCC;
use sqlx::{MySql, Row};

use crate::{
  common::{Language, ServerType, COMMAND_LINE, POOL},
  data::mangos,
};

use super::TranslateTarget;

async fn data_count(
  translate_target: &TranslateTarget,
  origin_language: Language,
) -> anyhow::Result<Vec<(String, i64)>> {
  let TranslateTarget {
    database,
    table,
    locale_columns,
  } = translate_target;
  let mut counts = vec![];

  for locale_column in locale_columns {
    let origin_locale_colume = mangos::column_name(locale_column, origin_language);
    let target_locale_column = mangos::column_name(locale_column, !origin_language);

    let count: i64 = sqlx::query::<MySql>(&format!(
      "SELECT count(*) FROM {database}.{table} WHERE {origin_locale_colume} IS NOT NULL AND {origin_locale_colume} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '')"))
      .fetch_one(&*POOL)
      .await?
      .get("count(*)");

    counts.push((table.clone(), count));
  }

  Ok(counts)
}

async fn translate_column(
  translate_target: &TranslateTarget,
  origin_language: Language,
  origin_count: i64,
  locale_column: String,
) -> anyhow::Result<()> {
  let TranslateTarget {
    database, table, ..
  } = translate_target;

  let origin_locale_colume = mangos::column_name(&locale_column, origin_language);
  let target_locale_column = mangos::column_name(locale_column, !origin_language);

  info!(
    "Translating table {database}.{table} from {origin_language} (total count: {origin_count}) ..."
  );

  let (mut translate_rows_count, batch_size) = (0, COMMAND_LINE.batch_size);
  for i in (0..origin_count).step_by(COMMAND_LINE.batch_size) {
    let results = sqlx::query::<MySql>(&format!(
      "SELECT entry,{origin_locale_colume},{target_locale_column} FROM {database}.{table} WHERE {origin_locale_colume} IS NOT NULL AND {origin_locale_colume} != '' AND ({target_locale_column} IS NULL OR {target_locale_column} = '') LIMIT {batch_size}"
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

  info!("Translate table {database}.{table} from {origin_locale_colume} to {target_locale_column} finished (translate rows count: {translate_rows_count}/{origin_count}) ...");

  Ok(())
}

async fn translate_language(
  origin_language: Language,
  translate_target: &TranslateTarget,
) -> anyhow::Result<()> {
  let origin_counts = data_count(translate_target, origin_language).await?;

  for (column, origin_count) in origin_counts {
    if origin_count > 0 {
      translate_column(translate_target, origin_language, origin_count, column).await?;
    }
  }

  Ok(())
}

pub async fn translate_tables(server_type: &ServerType) -> anyhow::Result<()> {
  let targets = mangos::get_translate_targets(server_type);

  for translate_target in &targets {
    translate_language(Language::Chinese, translate_target).await?;
    translate_language(Language::Taiwanese, translate_target).await?;
  }

  Ok(())
}
