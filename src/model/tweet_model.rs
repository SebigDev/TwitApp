use crate::dtos::dto::TweetDto;
use crate::model::like_model::Like;
use crate::model::tweet_comment::Comment;
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
    pub comments: Vec<Comment>,
}

impl Tweet {
    pub fn new(message: &str) -> Tweet {
        Tweet {
            id: None,
            created_at: Utc::now(),
            message: message.to_string(),
            likes: vec![],
            comments: vec![],
        }
    }

    pub fn map(&self) -> TweetDto {
        TweetDto {
            id: self.id.unwrap().to_hex(),
            created_at: self.created_at,
            message: self.message.clone(),
            likes: self.likes.clone().into_iter().map(|l| l.map()).collect(),
            comments: self.comments.clone().into_iter().map(|c| c.map()).collect(),
        }
    }
    pub fn add_like(&mut self, like: Like) {
        self.likes.push(like);
    }

    pub fn remove_like(&mut self, id: &str) {
        let _id = ObjectId::parse_str(id).unwrap();
        self.likes
            .retain(|a| !a.id.unwrap().to_hex().eq(&_id.to_hex()));
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment)
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
