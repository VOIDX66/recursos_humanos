use serde::{ Serialize, Deserialize };

#[derive(Serialize)]
pub struct EvaluationResponse {
    pub id: String,
    pub vacancy_id: String,
    pub candidate_id: String,
    pub id_number: String,
    pub evaluation_date: String,
    pub feedback: Option<String>,
    pub score: Option<f64>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct GradeEvaluationInput {
    pub evaluation_id: String,
    pub feedback: String,
    pub score: f64,
}