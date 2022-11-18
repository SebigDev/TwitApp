use actix_web::web;

use crate::api::{
    like_api::{delete_like, get_like, list_likes, plus_one},
    tweet_api::{create_tweet, get_tweet, list_tweets},
};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(create_tweet);
    config.service(list_tweets);
    config.service(get_tweet);
    config.service(delete_like);

    config.service(plus_one);
    config.service(list_likes);
    config.service(get_like);
    config.service(delete_like);
}
