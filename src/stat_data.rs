use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    // #[serde(rename = "gamesPlayed")]
    // pub games_played: Option<i64>,
    // pub wins: FaceOffsLost,
    // pub losses: FaceOffsLost,
    // pub ot: FaceOffsLost,
    // pub pts: FaceOffsLost,
    // #[serde(rename = "ptPctg")]
    // pub pt_pctg: String,
    #[serde(rename = "goalsPerGame")]
    pub goals_per_game: f64,
    #[serde(rename = "goalsAgainstPerGame")]
    pub goals_against_per_game: f64,
    // #[serde(rename = "evGGARatio")]
    // pub ev_gga_ratio: EvGgaRatio,
    #[serde(rename = "powerPlayPercentage")]
    pub power_play_percentage: String,
    // #[serde(rename = "powerPlayGoals")]
    // pub power_play_goals: FaceOffsLost,
    // #[serde(rename = "powerPlayGoalsAgainst")]
    // pub power_play_goals_against: FaceOffsLost,
    // #[serde(rename = "powerPlayOpportunities")]
    // pub power_play_opportunities: FaceOffsLost,
    #[serde(rename = "penaltyKillPercentage")]
    pub penalty_kill_percentage: String,
    // #[serde(rename = "shotsPerGame")]
    // pub shots_per_game: EvGgaRatio,
    // #[serde(rename = "shotsAllowed")]
    // pub shots_allowed: EvGgaRatio,
    // #[serde(rename = "winScoreFirst")]
    // pub win_score_first: EvGgaRatio,
    // #[serde(rename = "winOppScoreFirst")]
    // pub win_opp_score_first: EvGgaRatio,
    // #[serde(rename = "winLeadFirstPer")]
    // pub win_lead_first_per: EvGgaRatio,
    // #[serde(rename = "winLeadSecondPer")]
    // pub win_lead_second_per: FaceOffsLost,
    // #[serde(rename = "winOutshootOpp")]
    // pub win_outshoot_opp: EvGgaRatio,
    // #[serde(rename = "winOutshotByOpp")]
    // pub win_outshot_by_opp: EvGgaRatio,
    // #[serde(rename = "faceOffsTaken")]
    // pub face_offs_taken: FaceOffsLost,
    // #[serde(rename = "faceOffsWon")]
    // pub face_offs_won: FaceOffsLost,
    // #[serde(rename = "faceOffsLost")]
    // pub face_offs_lost: FaceOffsLost,
    #[serde(rename = "faceOffWinPercentage")]
    pub face_off_win_percentage: String,
    #[serde(rename = "shootingPctg")]
    pub shooting_pctg: f64,
    #[serde(rename = "savePctg")]
    pub save_pctg: f64,
    // #[serde(rename = "penaltyKillOpportunities")]
    // pub penalty_kill_opportunities: Option<String>,
    // #[serde(rename = "savePctRank")]
    // pub save_pct_rank: Option<String>,
    // #[serde(rename = "shootingPctRank")]
    // pub shooting_pct_rank: Option<String>,
}
