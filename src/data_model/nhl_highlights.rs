use serde::{Deserialize, Serialize};
use crate::data_model::nhl_highlight_items::NhlHighlightItems;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlHighlights {
    pub game_center: NhlHighlightItems,
}