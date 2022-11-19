use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use mongodb::Collection;

use crate::dtos::dto::TweetDto;
use crate::model::docs::update_tweet_document;
use crate::model::like_model::Like;
use crate::model::tweet_model::Tweet;

pub struct TweetRepo<Tweet> {
    pub collection: Collection<Tweet>,
}

impl TweetRepo<Tweet> {
    pub async fn create_tweet(&self, tweet: Tweet) -> Result<TweetDto, Error> {
        let _tweet = self
            .collection
            .insert_one(tweet, None)
            .await
            .ok()
            .expect("Error creating like");

        let id_option = _tweet.inserted_id.as_str();
        let id = match id_option {
            Some(id) => id,
            None => "",
        };
        let dto = self.get_tweet(id).await.unwrap();
        Ok(dto)
    }

    pub async fn all_tweets(&self) -> Result<Vec<TweetDto>, Error> {
        let mut _tweets = self
            .collection
            .find(None, None)
            .await
            .ok()
            .expect("Failed to retrieve all likes");
        let mut tweets = Vec::<Tweet>::new();
        while _tweets.advance().await.unwrap() {
            tweets.push(_tweets.deserialize_current().unwrap());
        }
        let dto = tweets
            .into_iter()
            .map(|t| t.map())
            .collect::<Vec<TweetDto>>();
        Ok(dto)
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
        let tweet_dto: TweetDto = _tweet.deserialize_current().unwrap().map();
        Ok(tweet_dto)
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

    pub async fn create_like(&self, tweet_id: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let tweet_dto = self.get_tweet(tweet_id).await.unwrap();
        let mut tweet = tweet_dto.to_tweet();
        tweet.add_like(Like::new(tweet_id));
        let query = doc! {"_id": _id };
        let _like = self
            .collection
            .update_one(query, update_tweet_document(&tweet), None)
            .await
            .ok()
            .expect("Error creating like");
        Ok(tweet.map())
    }
}
