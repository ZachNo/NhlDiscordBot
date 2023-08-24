use serde::{Deserialize, Serialize};
use crate::data_model::nhl_highlight_media::NhlHighlightMedia;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlHighlightItems {
    pub items: Vec<NhlHighlightMedia>,
}