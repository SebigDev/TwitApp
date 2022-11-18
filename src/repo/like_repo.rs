use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use mongodb::Collection;

use crate::dtos::dto::LikeDto;
use crate::model::like_model::Like;

pub struct LikeRepo<Like> {
    pub collection: Collection<Like>,

}

impl LikeRepo<Like> {
    pub async fn create_like(&self, tweet_id: &str) -> Result<LikeDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let new_like = Like {
            id: None,
            created_at: Utc::now(),
            tweet_id: Some(_id),
        };
        let _like = self
            .collection
            .insert_one(new_like, None)
            .await
            .ok()
            .expect("Error creating like");
        let id = _like.inserted_id.as_object_id().unwrap().to_hex();
        Ok(self.get_like(&id).await.unwrap())
    }

    pub async fn all_likes(&self) -> Result<Vec<LikeDto>, Error> {
        let _likes = self
            .collection
            .find(None, None)
            .await
            .ok()
            .expect("Failed to retrieve all likes");
        let dto = _likes.deserialize_current().unwrap();
        Ok(vec![dto.map()])
    }

    pub async fn get_like(&self, id: &str) -> Result<LikeDto, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid like Id provided");
        let filter = doc! {"_id": _id};
        let _like = self
            .collection
            .find(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve like");
        Ok(_like.deserialize_current().ok().unwrap().map())
    }

    pub async fn delete_like(&self, id: &str) -> Result<DeleteResult, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid like Id provided");
        let filter = doc! {"_id": _id};
        let _like = self
            .collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve like");
        Ok(_like)
    }
}
