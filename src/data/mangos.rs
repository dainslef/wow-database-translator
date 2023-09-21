use crate::{
  common::{Language, ServerType},
  translate::TranslateTarget,
};

pub fn column_name(column_base_name: impl ToString, language: Language) -> String {
  match language {
    Language::Chinese => column_base_name.to_string() + "_loc4",
    Language::Taiwanese => column_base_name.to_string() + "_loc5",
  }
}

pub fn get_translate_targets(server_type: &ServerType) -> Vec<TranslateTarget> {
  [
    // ("locales_gossip_menu_option", "option_text"),
    // ("locales_gossip_menu_option", "box_text"),
    ("locales_gameobject", vec!["name"]),
    ("locales_points_of_interest", vec!["icon_name"]),
    ("locales_page_text", vec!["Text"]),
    ("locales_creature", vec!["name", "subname"]),
    ("locales_item", vec!["name", "description"]),
    (
      "locales_npc_text",
      vec![
        "Text0_0", "Text0_1", "Text1_0", "Text1_1", "Text2_0", "Text2_1", "Text3_0", "Text3_1",
        "Text4_0", "Text4_1", "Text5_0", "Text5_1", "Text6_0", "Text6_1", "Text7_0", "Text7_1",
      ],
    ),
    (
      "locales_quest",
      vec![
        "Title",
        "Details",
        "Objectives",
        "OfferRewardText",
        "EndText",
        "ObjectiveText1",
        "ObjectiveText2",
        "ObjectiveText3",
        "ObjectiveText4",
      ],
    ),
  ]
  .into_iter()
  .map(|(table, locale_column)| {
    TranslateTarget::multi_columns(
      server_type,
      table,
      locale_column.into_iter().map(|v| v.to_string()).collect(),
    )
  })
  .collect()
}
