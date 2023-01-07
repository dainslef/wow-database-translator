use crate::{common::Language, translate::*};
use const_format::formatcp;
use sqlx::{mysql::MySqlArguments, query::Query, MySql};

#[derive(sqlx::FromRow, Debug)]
pub struct QuestTemplateLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Title")]
  pub title: String,
  #[sqlx(rename = "Details")]
  pub details: String,
  #[sqlx(rename = "Objectives")]
  pub objectives: String,
  #[sqlx(rename = "EndText")]
  pub end_text: String,
  #[sqlx(rename = "CompletedText")]
  pub completed_text: String,
  #[sqlx(rename = "ObjectiveText1")]
  pub objective_text_1: String,
  #[sqlx(rename = "ObjectiveText2")]
  pub objective_text_2: String,
  #[sqlx(rename = "ObjectiveText3")]
  pub objective_text_3: String,
  #[sqlx(rename = "ObjectiveText4")]
  pub objective_text_4: String,
}

impl TranslateLogic for QuestTemplateLocale {
  const TARGET: TranslateTarget =
    TranslateTarget::new("acore_world", "quest_template_locale", "locale");

  const SQL: &'static str = formatcp!(
    "INSERT IGNORE INTO {}.{} (ID, locale, Title, Details, Objectives, EndText, CompletedText,
      ObjectiveText1, ObjectiveText2, ObjectiveText3, ObjectiveText4)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    QuestTemplateLocale::TARGET.database,
    QuestTemplateLocale::TARGET.table
  );

  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments> {
    let opencc = self.locale.opencc();
    sqlx::query(Self::SQL)
      .bind(self.id)
      .bind(self.locale.target().to_string())
      .bind(opencc.convert(&self.title))
      .bind(opencc.convert(&self.details))
      .bind(opencc.convert(&self.objectives))
      .bind(opencc.convert(&self.end_text))
      .bind(opencc.convert(&self.completed_text))
      .bind(opencc.convert(&self.objective_text_1))
      .bind(opencc.convert(&self.objective_text_2))
      .bind(opencc.convert(&self.objective_text_3))
      .bind(opencc.convert(&self.objective_text_4))
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct AchievementRewardLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(rename = "Locale")]
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Subject")]
  pub subject: String,
  #[sqlx(rename = "Text")]
  pub text: String,
}

impl TranslateLogic for AchievementRewardLocale {
  const TARGET: TranslateTarget =
    TranslateTarget::new("acore_world", "achievement_reward_locale", "Locale");

  const SQL: &'static str = formatcp!(
    "INSERT IGNORE INTO {}.{} (ID, Locale, Subject, Text) VALUES (?, ?, ?, ?)",
    AchievementRewardLocale::TARGET.database,
    AchievementRewardLocale::TARGET.table
  );

  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments> {
    let opencc = self.locale.opencc();
    sqlx::query(Self::SQL)
      .bind(self.id)
      .bind(self.locale.target().to_string())
      .bind(opencc.convert(&self.subject))
      .bind(opencc.convert(&self.text))
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct BroadcastTextLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "MaleText")]
  pub male_text: String,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i16,
}

impl TranslateLogic for BroadcastTextLocale {
  const TARGET: TranslateTarget =
    TranslateTarget::new("acore_world", "broadcast_text_locale", "locale");

  const SQL: &'static str = formatcp!(
    "INSERT IGNORE INTO {}.{} (ID, locale, MaleText, VerifiedBuild) VALUES (?, ?, ?, ?)",
    BroadcastTextLocale::TARGET.database,
    BroadcastTextLocale::TARGET.table
  );

  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments> {
    sqlx::query(Self::SQL)
      .bind(self.id)
      .bind(self.locale.target().to_string())
      .bind(self.locale.opencc().convert(&self.male_text))
      .bind(self.verified_build)
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CreatureTemplateLocale {
  pub entry: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Name")]
  pub name: String,
  #[sqlx(rename = "Title")]
  pub title: String,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i16,
}

impl TranslateLogic for CreatureTemplateLocale {
  const TARGET: TranslateTarget =
    TranslateTarget::new("acore_world", "creature_template_locale", "locale");

  const SQL: &'static str = formatcp!(
    "INSERT IGNORE INTO {}.{} (entry, locale, Name, Title, VerifiedBuild) VALUES (?, ?, ?, ?, ?)",
    CreatureTemplateLocale::TARGET.database,
    CreatureTemplateLocale::TARGET.table
  );

  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments> {
    let opencc = self.locale.opencc();
    sqlx::query(Self::SQL)
      .bind(self.entry)
      .bind(self.locale.target().to_string())
      .bind(opencc.convert(&self.name))
      .bind(opencc.convert(&self.title))
      .bind(self.verified_build)
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CreatureTextLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(rename = "GroupID")]
  pub group_id: u32,
  #[sqlx(rename = "CreatureID")]
  pub creature_id: u32,
  #[sqlx(try_from = "String")]
  #[sqlx(rename = "Locale")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Text")]
  pub text: String,
}

impl TranslateLogic for CreatureTextLocale {
  const TARGET: TranslateTarget =
    TranslateTarget::new("acore_world", "creature_text_locale", "locale");

  const SQL: &'static str = formatcp!(
    "INSERT IGNORE INTO {}.{} (ID, GroupID, CreatureID, Locale, Text) VALUES (?, ?, ?, ?, ?)",
    CreatureTextLocale::TARGET.database,
    CreatureTextLocale::TARGET.table,
  );

  fn bind_query(&self) -> Query<'static, MySql, MySqlArguments> {
    sqlx::query(Self::SQL)
      .bind(self.id)
      .bind(self.group_id)
      .bind(self.creature_id)
      .bind(self.locale.target().to_string())
      .bind(self.locale.opencc().convert(&self.text))
  }
}
