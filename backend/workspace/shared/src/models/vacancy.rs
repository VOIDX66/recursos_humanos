use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Clone)]
pub struct Vacancy {
    pub id: String,
    pub title: String,
    pub description: String,
    pub requirements: Option<String>,
    pub salary: Option<f64>,
    pub opening_date: DateTime<Utc>,          // fecha con zona horaria
    pub closing_date: Option<DateTime<Utc>>,  // opcional, con zona horaria
    pub status: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewVacancy {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,

    pub requirements: Option<String>,
    pub salary: Option<f64>,
    pub closing_date: Option<DateTime<Utc>>,  // sin NaiveDate
}