use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    model::{
        tweet_comment::{CommentAction, CommentRequest},
        tweet_model::{Tweet, TweetActions, TweetRequest},
    },
    repo::tweet_repo::TweetRepo,
};

#[post("/api/v1/tweets")]
pub async fn create_tweet(
    request: Json<TweetRequest>,
    db: Data<TweetRepo<Tweet>>,
) -> impl Responder {
    let tweet = request.0.tweet().unwrap();
    let result = db.create_tweet(tweet).await;

    match result {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/v1/tweets")]
pub async fn list_tweets(db: Data<TweetRepo<Tweet>>) -> impl Responder {
    let result = db.all_tweets().await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/v1/tweets/{path}")]
pub async fn get_tweet(db: Data<TweetRepo<Tweet>>, path: Path<(String,)>) -> impl Responder {
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided"));
    }
    let result = db.get_tweet(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/api/v1/tweets/{path}")]
pub async fn delete_tweet(db: Data<TweetRepo<Tweet>>, path: Path<(String,)>) -> impl Responder {
    println!("Path {}", path.0.as_str());
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided").to_string());
    }
    let result = db.delete_tweet(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/v1/tweets/{path}/comment")]
pub async fn add_comment(db: Data<TweetRepo<Tweet>>, path: Path<(String,)>, request: Json<CommentRequest>,
) -> impl Responder {
    let tweet_id = path.0.as_str();
    let comment = request.0.comment(tweet_id).unwrap();
    let result = db.add_comment(tweet_id, &comment.message).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
