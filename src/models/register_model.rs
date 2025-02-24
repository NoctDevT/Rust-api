use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub description: String,
}


#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}
