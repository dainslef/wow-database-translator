use crate::{
  common::{ConvertText, Language},
  translate::*,
};
use once_cell::sync::Lazy;
use opencc_rust::OpenCC;
use sqlx::{MySql, QueryBuilder};

#[derive(sqlx::FromRow, Debug)]
pub struct AchievementRewardLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(rename = "Locale")]
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Subject")]
  pub subject: Option<String>,
  #[sqlx(rename = "Text")]
  pub text: Option<String>,
}

impl TranslateLogic for AchievementRewardLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "achievement_reward_locale", "Locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();

    // let sql = format!(
    //   "INSERT IGNORE INTO {}.{} (ID, Locale, Subject, Text) VALUES (?, ?, ?, ?)",
    //   Self::TARGET.database,
    //   Self::TARGET.table
    // );
    // sqlx::query(&sql)
    //   .bind(self.id)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.subject))
    //   .bind(opencc.convert_text(&self.text))

    let mut builder = QueryBuilder::new(format!(
      "INSERT IGNORE INTO {}.{} (ID, Locale, Subject, Text) VALUES ",
      Self::TARGET.database,
      Self::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.subject))
      .push_bind(opencc.convert_text(&self.text))
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct BroadcastTextLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "MaleText")]
  pub male_text: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for BroadcastTextLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "broadcast_text_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // let sql = format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, MaleText, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   BroadcastTextLocale::TARGET.database,
    //   BroadcastTextLocale::TARGET.table
    // );
    // sqlx::query(&sql)
    //   .bind(self.id)
    //   .bind(!self.locale)
    //   .bind(self.locale.convert_text(&self.male_text))
    //   .bind(self.verified_build)

    let mut builder = QueryBuilder::new(format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, MaleText, VerifiedBuild) VALUES ",
      Self::TARGET.database,
      Self::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(self.locale.convert_text(&self.male_text))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CreatureTemplateLocale {
  pub entry: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Name")]
  pub name: Option<String>,
  #[sqlx(rename = "Title")]
  pub title: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for CreatureTemplateLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "creature_template_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // let sql = format!(
    //   "INSERT IGNORE INTO {}.{} (entry, locale, Name, Title, VerifiedBuild) VALUES (?, ?, ?, ?, ?)",
    //   CreatureTemplateLocale::TARGET.database,
    //   CreatureTemplateLocale::TARGET.table
    // );
    // sqlx::query(&sql)
    //   .bind(self.entry)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.name))
    //   .bind(opencc.convert_text(&self.title))
    //   .bind(self.verified_build)

    let mut builder = QueryBuilder::new(format!(
      "INSERT IGNORE INTO {}.{} (entry, locale, Name, Title, VerifiedBuild) VALUES ",
      Self::TARGET.database,
      Self::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.entry)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.name))
      .push_bind(opencc.convert_text(&self.title))
      .push_unseparated(")");

    builder
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
  pub text: Option<String>,
}

impl TranslateLogic for CreatureTextLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "creature_text_locale", "Locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, GroupID, CreatureID, Locale, Text) VALUES (?, ?, ?, ?, ?)",
    //   CreatureTextLocale::TARGET.database,
    //   CreatureTextLocale::TARGET.table,
    // ))
    // .bind(self.id)
    // .bind(self.group_id)
    // .bind(self.creature_id)
    // .bind(!self.locale)
    // .bind(self.locale.convert_text(&self.text))

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, GroupID, CreatureID, Locale, Text) VALUES ",
      CreatureTextLocale::TARGET.database,
      CreatureTextLocale::TARGET.table,
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(self.group_id)
      .push_bind(self.creature_id)
      .push_bind(!self.locale)
      .push_bind(self.locale.convert_text(&self.text))
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct GameobjectTemplateLocale {
  pub entry: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  pub name: Option<String>,
  #[sqlx(rename = "castBarCaption")]
  pub cast_bar_caption: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for GameobjectTemplateLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "gameobject_template_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (entry, locale, name, castBarCaption, VerifiedBuild) VALUES (?, ?, ?, ?, ?)",
    //   GameobjectTemplateLocale::TARGET.database,
    //   GameobjectTemplateLocale::TARGET.table
    // ))
    //   .bind(self.entry)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.name))
    //   .bind(opencc.convert_text(&self.cast_bar_caption))
    //   .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (entry, locale, name, castBarCaption, VerifiedBuild) VALUES ",
      GameobjectTemplateLocale::TARGET.database,
      GameobjectTemplateLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.entry)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.name))
      .push_bind(opencc.convert_text(&self.cast_bar_caption))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct GossipMenuOptionLocale {
  #[sqlx(rename = "MenuID")]
  pub menu_id: u32,
  #[sqlx(rename = "OptionID")]
  pub option_id: u32,
  #[sqlx(try_from = "String")]
  #[sqlx(rename = "Locale")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "OptionText")]
  pub option_text: Option<String>,
  #[sqlx(rename = "BoxText")]
  pub box_text: Option<String>,
}

impl TranslateLogic for GossipMenuOptionLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "gossip_menu_option_locale", "Locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // sqlx::query(&    format!(
    //   "INSERT IGNORE INTO {}.{} (MenuID, OptionID, Locale, OptionText, BoxText) VALUES (?, ?, ?, ?, ?)",
    //   GossipMenuOptionLocale::TARGET.database,
    //   GossipMenuOptionLocale::TARGET.table,
    // ))
    //   .bind(self.menu_id)
    //   .bind(self.option_id)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.option_text))
    //   .bind(opencc.convert_text(&self.box_text))

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (MenuID, OptionID, Locale, OptionText, BoxText) VALUES ",
      GossipMenuOptionLocale::TARGET.database,
      GossipMenuOptionLocale::TARGET.table,
    ));
    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.menu_id)
      .push_bind(self.option_id)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.option_text))
      .push_bind(opencc.convert_text(&self.box_text))
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct ItemSetNamesLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Name")]
  pub name: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for ItemSetNamesLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "item_set_names_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, Name, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   ItemSetNamesLocale::TARGET.database,
    //   ItemSetNamesLocale::TARGET.table,
    // ))
    // .bind(self.id)
    // .bind(!self.locale)
    // .bind(self.locale.convert_text(&self.name))
    // .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, Name, VerifiedBuild) VALUES ",
      ItemSetNamesLocale::TARGET.database,
      ItemSetNamesLocale::TARGET.table,
    ));
    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(self.locale.convert_text(&self.name))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct ItemTemplateLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Name")]
  pub name: Option<String>,
  #[sqlx(rename = "Description")]
  pub description: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for ItemTemplateLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "item_template_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // sqlx::query(&  format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, Name, Description, VerifiedBuild) VALUES (?, ?, ?, ?, ?)",
    //   ItemTemplateLocale::TARGET.database,
    //   ItemTemplateLocale::TARGET.table,
    // ))
    //   .bind(self.id)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.name))
    //   .bind(opencc.convert_text(&self.description))
    //   .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, Name, Description, VerifiedBuild) VALUES ",
      ItemTemplateLocale::TARGET.database,
      ItemTemplateLocale::TARGET.table,
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.name))
      .push_bind(opencc.convert_text(&self.description))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct NpcTextLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(rename = "Locale")]
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Text0_0")]
  pub text0_0: Option<String>,
  #[sqlx(rename = "Text0_1")]
  pub text0_1: Option<String>,
  #[sqlx(rename = "Text1_0")]
  pub text1_0: Option<String>,
  #[sqlx(rename = "Text1_1")]
  pub text1_1: Option<String>,
  #[sqlx(rename = "Text2_0")]
  pub text2_0: Option<String>,
  #[sqlx(rename = "Text2_1")]
  pub text2_1: Option<String>,
  #[sqlx(rename = "Text3_0")]
  pub text3_0: Option<String>,
  #[sqlx(rename = "Text3_1")]
  pub text3_1: Option<String>,
  #[sqlx(rename = "Text4_0")]
  pub text4_0: Option<String>,
  #[sqlx(rename = "Text4_1")]
  pub text4_1: Option<String>,
  #[sqlx(rename = "Text5_0")]
  pub text5_0: Option<String>,
  #[sqlx(rename = "Text5_1")]
  pub text5_1: Option<String>,
  #[sqlx(rename = "Text6_0")]
  pub text6_0: Option<String>,
  #[sqlx(rename = "Text6_1")]
  pub text6_1: Option<String>,
  #[sqlx(rename = "Text7_0")]
  pub text7_0: Option<String>,
  #[sqlx(rename = "Text7_1")]
  pub text7_1: Option<String>,
}

impl TranslateLogic for NpcTextLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "npc_text_locale", "Locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // sqlx::query(&    format!(
    //   "INSERT IGNORE INTO {}.{} (ID, Locale, Text0_0, Text0_1, Text1_0, Text1_1, Text2_0, Text2_1, Text3_0,Text3_1, Text4_0, Text4_1, Text5_0, Text5_1, Text6_0, Text6_1, Text7_0, Text7_1) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    //   NpcTextLocale::TARGET.database,
    //   NpcTextLocale::TARGET.table,
    // ))
    //   .bind(self.id)
    //   .bind(!self.locale)
    //   .bind(opencc.convert_text(&self.text0_0))
    //   .bind(opencc.convert_text(&self.text0_1))
    //   .bind(opencc.convert_text(&self.text1_0))
    //   .bind(opencc.convert_text(&self.text1_1))
    //   .bind(opencc.convert_text(&self.text2_0))
    //   .bind(opencc.convert_text(&self.text2_1))
    //   .bind(opencc.convert_text(&self.text3_0))
    //   .bind(opencc.convert_text(&self.text3_1))
    //   .bind(opencc.convert_text(&self.text4_0))
    //   .bind(opencc.convert_text(&self.text4_1))
    //   .bind(opencc.convert_text(&self.text5_0))
    //   .bind(opencc.convert_text(&self.text5_1))
    //   .bind(opencc.convert_text(&self.text6_0))
    //   .bind(opencc.convert_text(&self.text6_1))
    //   .bind(opencc.convert_text(&self.text7_0))
    //   .bind(opencc.convert_text(&self.text7_1))

    let mut builder = sqlx::QueryBuilder::new(&    format!(
      "INSERT IGNORE INTO {}.{} (ID, Locale, Text0_0, Text0_1, Text1_0, Text1_1, Text2_0, Text2_1, Text3_0,Text3_1, Text4_0, Text4_1, Text5_0, Text5_1, Text6_0, Text6_1, Text7_0, Text7_1) VALUES ",
      NpcTextLocale::TARGET.database,
      NpcTextLocale::TARGET.table,
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(opencc.convert_text(&self.text0_0))
      .push_bind(opencc.convert_text(&self.text0_1))
      .push_bind(opencc.convert_text(&self.text1_0))
      .push_bind(opencc.convert_text(&self.text1_1))
      .push_bind(opencc.convert_text(&self.text2_0))
      .push_bind(opencc.convert_text(&self.text2_1))
      .push_bind(opencc.convert_text(&self.text3_0))
      .push_bind(opencc.convert_text(&self.text3_1))
      .push_bind(opencc.convert_text(&self.text4_0))
      .push_bind(opencc.convert_text(&self.text4_1))
      .push_bind(opencc.convert_text(&self.text5_0))
      .push_bind(opencc.convert_text(&self.text5_1))
      .push_bind(opencc.convert_text(&self.text6_0))
      .push_bind(opencc.convert_text(&self.text6_1))
      .push_bind(opencc.convert_text(&self.text7_0))
      .push_bind(opencc.convert_text(&self.text7_1))
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct PageTextLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Text")]
  pub text: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for PageTextLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "page_text_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, Text, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   PageTextLocale::TARGET.database,
    //   PageTextLocale::TARGET.table
    // ))
    // .bind(self.id)
    // .bind(!self.locale)
    // .bind(self.locale.convert_text(&self.text))
    // .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, Text, VerifiedBuild) VALUES ",
      PageTextLocale::TARGET.database,
      PageTextLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(!self.locale)
      .push_bind(self.locale.convert_text(&self.text))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct PointsOfInterestLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Name")]
  pub name: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for PointsOfInterestLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "points_of_interest_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // let sql = format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, Name, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   PointsOfInterestLocale::TARGET.database,
    //   PointsOfInterestLocale::TARGET.table
    // );
    // sqlx::query(&sql)
    //   .bind(self.id)
    //   .bind((!self.locale).to_string())
    //   .bind(self.locale.convert_text(&self.name))
    //   .bind(self.verified_build)

    let sql = format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, Name, VerifiedBuild) VALUES ",
      PointsOfInterestLocale::TARGET.database,
      PointsOfInterestLocale::TARGET.table
    );
    let mut builder = sqlx::QueryBuilder::new(&sql);

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind((!self.locale).to_string())
      .push_bind(self.locale.convert_text(&self.name))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct QuestGreetingLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  pub r#type: u8,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Greeting")]
  pub greeting: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for QuestGreetingLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "quest_greeting_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, type, locale, Greeting, VerifiedBuild) VALUES (?, ?, ?, ?, ?)",
    //   QuestGreetingLocale::TARGET.database,
    //   QuestGreetingLocale::TARGET.table
    // ))
    // .bind(self.id)
    // .bind(self.r#type)
    // .bind(!self.locale)
    // .bind(self.locale.convert_text(&self.greeting))
    // .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, type, locale, Greeting, VerifiedBuild) VALUES ",
      QuestGreetingLocale::TARGET.database,
      QuestGreetingLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind(self.r#type)
      .push_bind(!self.locale)
      .push_bind(self.locale.convert_text(&self.greeting))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct QuestOfferRewardLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "RewardText")]
  pub reward_text: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for QuestOfferRewardLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "quest_offer_reward_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, RewardText, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   QuestOfferRewardLocale::TARGET.database,
    //   QuestOfferRewardLocale::TARGET.table
    // ))
    // .bind(self.id)
    // .bind((!self.locale).to_string())
    // .bind(self.locale.convert_text(&self.reward_text))
    // .bind(self.verified_build)
    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, RewardText, VerifiedBuild) VALUES ",
      QuestOfferRewardLocale::TARGET.database,
      QuestOfferRewardLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind((!self.locale).to_string())
      .push_bind(self.locale.convert_text(&self.reward_text))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct QuestRequestItemsLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "CompletionText")]
  pub completion_text: Option<String>,
  #[sqlx(rename = "VerifiedBuild")]
  pub verified_build: i32,
}

impl TranslateLogic for QuestRequestItemsLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "quest_request_items_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, CompletionText, VerifiedBuild) VALUES (?, ?, ?, ?)",
    //   QuestRequestItemsLocale::TARGET.database,
    //   QuestRequestItemsLocale::TARGET.table
    // ))
    // .bind(self.id)
    // .bind((!self.locale).to_string())
    // .bind(self.locale.convert_text(&self.completion_text))
    // .bind(self.verified_build)

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, CompletionText, VerifiedBuild) VALUES ",
      QuestRequestItemsLocale::TARGET.database,
      QuestRequestItemsLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind((!self.locale).to_string())
      .push_bind(self.locale.convert_text(&self.completion_text))
      .push_bind(self.verified_build)
      .push_unseparated(")");

    builder
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct QuestTemplateLocale {
  #[sqlx(rename = "ID")]
  pub id: u32,
  #[sqlx(try_from = "String")]
  pub locale: Language, // Use try_from attribute for type convertion.
  #[sqlx(rename = "Title")]
  pub title: Option<String>,
  #[sqlx(rename = "Details")]
  pub details: Option<String>,
  #[sqlx(rename = "Objectives")]
  pub objectives: Option<String>,
  #[sqlx(rename = "EndText")]
  pub end_text: Option<String>,
  #[sqlx(rename = "CompletedText")]
  pub completed_text: Option<String>,
  #[sqlx(rename = "ObjectiveText1")]
  pub objective_text_1: Option<String>,
  #[sqlx(rename = "ObjectiveText2")]
  pub objective_text_2: Option<String>,
  #[sqlx(rename = "ObjectiveText3")]
  pub objective_text_3: Option<String>,
  #[sqlx(rename = "ObjectiveText4")]
  pub objective_text_4: Option<String>,
}

impl TranslateLogic for QuestTemplateLocale {
  const TARGET: Lazy<TranslateTarget> =
    Lazy::new(|| TranslateTarget::new("acore_world", "quest_template_locale", "locale"));

  fn build_query(&self) -> QueryBuilder<'static, MySql> {
    let opencc: &OpenCC = self.locale.into();
    // sqlx::query(&format!(
    //   "INSERT IGNORE INTO {}.{} (ID, locale, Title, Details, Objectives, EndText, CompletedText,
    //   ObjectiveText1, ObjectiveText2, ObjectiveText3, ObjectiveText4)
    //   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    //   QuestTemplateLocale::TARGET.database,
    //   QuestTemplateLocale::TARGET.table
    // ))
    // .bind(self.id)
    // .bind((!self.locale).to_string())
    // .bind(opencc.convert_text(&self.title))
    // .bind(opencc.convert_text(&self.details))
    // .bind(opencc.convert_text(&self.objectives))
    // .bind(opencc.convert_text(&self.end_text))
    // .bind(opencc.convert_text(&self.completed_text))
    // .bind(opencc.convert_text(&self.objective_text_1))
    // .bind(opencc.convert_text(&self.objective_text_2))
    // .bind(opencc.convert_text(&self.objective_text_3))
    // .bind(opencc.convert_text(&self.objective_text_4))

    let mut builder = sqlx::QueryBuilder::new(&format!(
      "INSERT IGNORE INTO {}.{} (ID, locale, Title, Details, Objectives, EndText, CompletedText,
      ObjectiveText1, ObjectiveText2, ObjectiveText3, ObjectiveText4) VALUES ",
      QuestTemplateLocale::TARGET.database,
      QuestTemplateLocale::TARGET.table
    ));

    builder
      .separated(",")
      .push_unseparated("(")
      .push_bind(self.id)
      .push_bind((!self.locale).to_string())
      .push_bind(opencc.convert_text(&self.title))
      .push_bind(opencc.convert_text(&self.details))
      .push_bind(opencc.convert_text(&self.objectives))
      .push_bind(opencc.convert_text(&self.end_text))
      .push_bind(opencc.convert_text(&self.completed_text))
      .push_bind(opencc.convert_text(&self.objective_text_1))
      .push_bind(opencc.convert_text(&self.objective_text_2))
      .push_bind(opencc.convert_text(&self.objective_text_3))
      .push_bind(opencc.convert_text(&self.objective_text_4))
      .push_unseparated(")");

    builder
  }
}
