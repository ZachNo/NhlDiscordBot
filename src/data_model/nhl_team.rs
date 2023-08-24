use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NhlTeam {
    pub name: String,
}