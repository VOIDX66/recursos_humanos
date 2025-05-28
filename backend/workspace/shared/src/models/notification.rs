use chrono::{DateTime, Utc};
use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Serialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}