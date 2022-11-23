use actix_web::web;

use crate::api::{
    like_api::{plus_one, minus_one},
    tweet_api::{create_tweet, get_tweet, list_tweets, add_comment},
};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(create_tweet);
    config.service(list_tweets);
    config.service(get_tweet);
    config.service(plus_one);
    config.service(minus_one);
    config.service(add_comment);
}
