extern crate actix_web;
extern crate diesel;
extern crate log;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};
mod dbconn;
mod like;
mod response;
mod schema;
mod tweet;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(dbconn::establish_connection())
            .wrap(middleware::Logger::default())
            .service(tweet::get)
            .service(tweet::create)
            .service(like::plus_one)
            .service(like::minus_one)
            .service(like::list)
    })
    .bind("127.0.0.1:8080")
    .expect("Address not found")
    .run()
    .await
}
