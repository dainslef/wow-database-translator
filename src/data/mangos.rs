use crate::common::Language;

pub fn column_name(column_base_name: impl ToString, language: Language) -> String {
  match language {
    Language::Chinese => column_base_name.to_string() + "_loc4",
    Language::Taiwanese => column_base_name.to_string() + "_loc5",
  }
}
