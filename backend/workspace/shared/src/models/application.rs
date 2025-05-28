use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub struct ApplicationRequest {
    #[serde(skip_deserializing)]
    pub user_id: String,
    pub vacancy_id: String
}

#[derive(Serialize)]
pub struct ApplicationView {
    pub application_id: String,
    pub applicant_name: String,
    pub applicant_email: String,
    pub vacancy_title: String,
    pub application_status : String,
    pub applied_at: DateTime<Utc>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DecideApplicationRequest {
    pub application_id: String,
    pub decision: String, // "pending_evaluation", "rejected", "accepted"
}