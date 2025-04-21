use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i32,
    pub id_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub rol: String
}

#[derive(Deserialize, Clone)]
pub struct NewUser {
    pub id_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub rol: String
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub id_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub rol: String
}