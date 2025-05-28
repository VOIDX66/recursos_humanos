use serde::{ Serialize, Deserialize };

#[derive(Serialize)]
pub struct EvaluationResponse {
    pub id: String,
    pub vacancy_id: String,
    pub candidate_id: String,
    pub evaluation_date: String,
    pub feedback: Option<String>,
    pub score: Option<f64>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct GradeEvaluationInput {
    pub feedback: String,
    pub score: f64,
}