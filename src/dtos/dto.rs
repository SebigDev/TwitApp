use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LikeDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub tweet_id: String,
}

#[derive(Debug, Serialize)]
pub struct TweetDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<String>,
}

