use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationRow {
    pub applicant_id: i32,
    pub program_id: i32,
    pub priority: i32,
    pub has_consent: bool,
    pub physics_score: i32,
    pub russian_score: i32,
    pub math_score: i32,
    pub individual_score: i32,
    pub total_score: i32,
}
