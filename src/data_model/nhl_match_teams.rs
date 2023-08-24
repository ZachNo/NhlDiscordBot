use serde::{Deserialize, Serialize};
use crate::data_model::nhl_match_team::NhlMatchTeam;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlMatchTeams {
    pub away: NhlMatchTeam,
    pub home: NhlMatchTeam,
}