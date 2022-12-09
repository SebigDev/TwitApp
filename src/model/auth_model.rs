use argonautica::{Hasher, Verifier};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};

use crate::auths::{auth::TokenClaim, utils::get_jwt_key};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: &str, password: &str) -> Self {
        User {
            id: None,
            created_at: Utc::now(),
            email: email.to_string(),
            password: Self::hash_password(password),
        }
    }

    pub fn hash_password(password: &str) -> String {
        let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY not provided");
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap();
        hash
    }

    pub fn generate_token(&self, password: &str) -> String {
        let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY not provided");
        let key = get_jwt_key();
        let mut verifier = Verifier::default();
        let verify = verifier
            .with_hash(&self.password)
            .with_password(password)
            .with_secret_key(secret)
            .verify()
            .unwrap();
        if verify {
            let claims = TokenClaim {
                id: self.id.unwrap().to_hex(),
                email: self.email.clone(),
            };
            let token_string = claims.sign_with_key(&key).unwrap();
            token_string
        } else {
            String::new()
        }
    }
}
