use crate::common::Language;

pub fn column_name(column_base_name: String, language: Language) -> String {
  match language {
    Language::Chinese => column_base_name + "_loc4",
    Language::Taiwanese => column_base_name + "_loc5",
  }
}
