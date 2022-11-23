use crate::model::tweet_model::Tweet;
use bson::doc;
use mongodb::bson::{self, Document};

pub fn update_tweet_document(tweet: &Tweet) -> Document {
    let _doc = doc! {
         "$set":{
            "_id": bson::Bson::ObjectId(tweet.id.unwrap()),
            "message": bson::Bson::String(tweet.message.clone()),
            "created_at": bson::to_bson(&tweet.created_at).unwrap(),
            "likes": bson::to_bson(&tweet.likes).unwrap(),
            "comments": bson::to_bson(&tweet.comments).unwrap()
         }
    };
    _doc
}
