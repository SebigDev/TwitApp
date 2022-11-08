use crate::like::Like;
use crate::response::Response;
use actix_web::{get, HttpResponse, post, web::Json, delete};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message: message.to_string(),
            likes: vec![],
        }
    }
}

#[get("/tweets")]
pub async fn get() -> HttpResponse {
    let tweets =  Tweets { results: vec![]};
    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

pub trait TweetActions {
    fn tweet(&self) -> Option<Tweet>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    message: Option<String>,
}

impl TweetActions for TweetRequest {
    fn tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message)),
            None => None,
        }
    }
}

#[post("/tweet")]
pub async fn create(tweet_request: Json<TweetRequest>) -> HttpResponse{
    HttpResponse::Created()
    .content_type("application/json")
    .json(tweet_request.tweet())
}

#[delete("/tweet/{id}")]
pub async fn delete() -> HttpResponse {
    HttpResponse::NoContent()
    .content_type("application/json")
    .await
    .unwrap()
}
