#[derive(serde::Deserialize)]
pub struct ApplicationRequest {
    #[serde(skip_deserializing)]
    pub user_id: String,
    pub vacancy_id: String
}