extern crate actix_web;
extern crate log;

use actix_web::{middleware, web::Data, App, HttpServer};
use dbconn::MongoPool;
use model::tweet_model::Tweet;
use repo::tweet_repo::TweetRepo;
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

    let db = MongoPool::<Tweet>::connect().await;
    let pool = Data::new(TweetRepo {
        collection: db.collection,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(pool.clone())
            .configure(router::init)
    })
    .bind("127.0.0.1:8080")
    .expect("Address not found")
    .run()
    .await
}
