use serde::{Deserialize, Serialize};
use crate::data_model::nhl_highlight_playback::NhlHighlightPlayback;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlHighlightMedia {
    #[serde(default)]
    #[serde(rename = "type")]
    pub highlight_type: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub playbacks: Vec<NhlHighlightPlayback>,
}