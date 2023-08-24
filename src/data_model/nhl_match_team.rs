use serde::{Deserialize, Serialize};
use crate::data_model::{
    nhl_match_team_stats::NhlMatchTeamStats,
    nhl_team::NhlTeam,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlMatchTeam {
    pub team: NhlTeam,
    pub team_stats: NhlMatchTeamStats,
}