use serde::{Deserialize, Serialize};
use crate::data_model::nhl_match_team_skater_stats::NhlMatchTeamSkaterStats;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlMatchTeamStats {
    pub team_skater_stats: NhlMatchTeamSkaterStats,
}