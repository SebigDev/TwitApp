extern crate actix_web;
extern crate log;

use actix_web::{ middleware, web::Data, App, HttpServer};
use dbconn::MongoPool;
use model::{like_model::Like, tweet_model::Tweet};
use repo::{like_repo::LikeRepo, tweet_repo::TweetRepo};
use routes::router;

use std::{env, io};
mod api;
mod dbconn;
mod dtos;
mod model;
mod repo;
mod routes;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

     let db = MongoPool::<Like>::connect().await;
     let pool = Data::new(LikeRepo { collection: db.collection });

     let db1 = MongoPool::<Tweet>::connect().await;
     let pool1 = Data::new(TweetRepo{ collection: db1.collection });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(pool.clone())
            .app_data(pool1.clone())
            .configure(router::init)
    })
    .bind("127.0.0.1:8080")
    .expect("Address not found")
    .run()
    .await
}
