use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

#[derive(Deserialize, Clone)]
pub struct NewUser {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}