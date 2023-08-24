use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NhlMatchTeamSkaterStats {
    pub goals: u32,
    pub pim: u32,
    pub shots: u32,
    pub power_play_percentage: String,
    pub power_play_goals: f32,
    pub power_play_opportunities: f32,
    pub face_off_win_percentage: String,
    pub blocked: u32,
    pub takeaways: u32,
    pub giveaways: u32,
    pub hits: u32,
}