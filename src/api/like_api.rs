use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse, Responder, delete,
};

use crate::{ model::like_model::Like, repo::like_repo::LikeRepo};

#[post("/api/v1/likes/{tweet_id}")]
pub async fn plus_one(db: Data<LikeRepo<Like>>, tweet_id: Path<(String,)>) -> impl Responder {
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

#[get("/api/v1/likes")]
pub async fn list_likes(db: Data<LikeRepo<Like>>) -> impl Responder {
    let result = db.all_likes().await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/v1/likes/{path}")]
pub async fn get_like(db: Data<LikeRepo<Like>>, path: Path<(String,)>) -> impl Responder {
    println!("Path {}", path.0.as_str());
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided"));
    }
    let result = db.get_like(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/api/v1/likes/{path}")]
pub async fn delete_like(db: Data<LikeRepo<Like>>, path: Path<(String,)>) -> impl Responder {
    println!("Path {}", path.0.as_str());
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided").to_string());
    }
    let result = db.delete_like(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
