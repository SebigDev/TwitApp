use crate::model::{like_model::Like, tweet_model::Tweet};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub tweet_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<LikeDto>
}

impl TweetDto {
    pub fn to_tweet(self) -> Tweet {
        Tweet {
            id: Some(ObjectId::parse_str(&self.id).unwrap()),
            message: self.message.to_owned(),
            created_at: self.created_at,
            likes: self.likes.into_iter().map(|l| l.to_like()).collect(),
        }
    }
}

impl LikeDto {
    pub fn to_like(&self) -> Like {
        Like {
            id: Some(ObjectId::parse_str(&self.id).unwrap()),
            created_at: self.created_at,
            tweet_id: Some(ObjectId::parse_str(&self.tweet_id).unwrap()),
        }
    }
}
