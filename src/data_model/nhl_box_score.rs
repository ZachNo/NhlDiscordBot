use serde::{Deserialize, Serialize};
use crate::data_model::nhl_match_teams::NhlMatchTeams;

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlBoxScore {
    pub teams: NhlMatchTeams,
}