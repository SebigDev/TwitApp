use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenClaim {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}
