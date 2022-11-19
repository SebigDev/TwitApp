use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{model::tweet_model::Tweet, repo::tweet_repo::TweetRepo};

#[post("/api/v1/likes/{tweet_id}")]
pub async fn plus_one(db: Data<TweetRepo<Tweet>>, tweet_id: Path<(String,)>) -> impl Responder {
    let id = tweet_id.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided"));
    }
    let result = db.create_like(id).await;

    match result {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
