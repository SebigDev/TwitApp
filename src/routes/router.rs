use actix_web::web;

use crate::api::{
    like_api::{minus_one, plus_one},
    tweet_api::{add_comment, create_tweet, delete_comment, get_tweet, list_tweets},
};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(create_tweet);
    config.service(list_tweets);
    config.service(get_tweet);
    config.service(plus_one);
    config.service(minus_one);
    config.service(add_comment);
    config.service(delete_comment);
}
