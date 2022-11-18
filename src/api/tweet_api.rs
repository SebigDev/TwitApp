use actix_web::{
    get, post,
    web::{Data, Path, Json},
    HttpResponse, Responder, delete,
};

use crate::{repo::tweet_repo::TweetRepo, model::tweet_model::{Tweet, TweetRequest, TweetActions}};


#[post("/api/v1/tweets")]
pub async fn create_tweet(request: Json<TweetRequest>, db: Data<TweetRepo<Tweet>>) -> impl Responder {
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
    println!("Path {}", path.0.as_str());
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
