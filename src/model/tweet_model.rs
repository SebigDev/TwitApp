use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::dtos::dto::TweetDto;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<String>
}

impl Tweet {
    pub fn new(message: &str) -> Tweet {
        Tweet {
            id: None,
            created_at: Utc::now(),
            message: message.to_string(),
            likes : vec![],
        }
    }

    pub fn map(&self) -> TweetDto {
        TweetDto {
            id: self.id.unwrap().to_hex(),
            created_at: self.created_at,
            message: self.message.clone(),
            likes: self.likes.clone(),
        }
    }
}

pub trait TweetActions {
   fn tweet(&self) -> Option<Tweet>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetActions for TweetRequest {
     fn tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message)),
            None => None,
        }
    }
}

