use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TranslationString {
    #[serde(default)]
    pub default: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: u32,
    pub common_name: TranslationString,
    pub place_name: TranslationString,
    pub score: Option<u8>,
    pub sog: Option<u8>,
}