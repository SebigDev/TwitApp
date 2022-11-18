use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::Collection;

use crate::dtos::dto::TweetDto;
use crate::model::tweet_model::Tweet;

pub struct TweetRepo<Tweet> {
    pub collection: Collection<Tweet>,
}

impl TweetRepo<Tweet> {
    pub async fn create_tweet(&self, tweet: Tweet) -> Result<InsertOneResult, Error> {
        let _tweet = self
            .collection
            .insert_one(tweet, None)
            .await
            .ok()
            .expect("Error creating like");
        Ok(_tweet)
    }

    pub async fn all_tweets(&self) -> Result<Vec<TweetDto>, Error> {
        let _tweets = self
            .collection
            .find(None, None)
            .await
            .ok()
            .expect("Failed to retrieve all likes");
        let dto = _tweets.deserialize_current().unwrap();
        Ok(vec![dto.map()])
    }

    pub async fn get_tweet(&self, id: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid like Id provided");
        let filter = doc! {"_id": _id};
        let _tweet = self
            .collection
            .find(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve like");
        Ok(_tweet.deserialize_current().ok().unwrap().map())
    }

    pub async fn delete_tweet(&self, id: &str) -> Result<DeleteResult, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid like Id provided");
        let filter = doc! {"_id": _id};
        let _tweet = self
            .collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve tweet");
        Ok(_tweet)
    }
}
