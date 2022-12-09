use crate::{
    auths::auth::AuthData, dtos::dto::UserDto, errors::error::TweetError, model::auth_model::User,
};
use bson::doc;
use mongodb::Collection;

pub struct UserRepo<User> {
    pub collection: Collection<User>,
}

impl UserRepo<User> {
    pub async fn register(&self, user: User) -> Result<UserDto, TweetError> {
        let _user = self
            .collection
            .insert_one(user, None)
            .await
            .map_err(|_| TweetError::InternalServerError)
            .ok()
            .unwrap();

        let id = match _user.inserted_id.as_object_id() {
            Some(id) => id.to_hex(),
            None => return Err(TweetError::BadRequest("Error registering user".into())),
        };
        return Ok(UserDto {
            id,
            message: "Your registration was successful".to_string(),
        });
    }

    pub async fn valid_user(&self, auth: AuthData) -> Result<String, TweetError> {
        let filter = doc! {"email": auth.email};
        let user_result = self
            .collection
            .find(filter, None)
            .await
            .map_err(|_| TweetError::InternalServerError)
            .ok()
            .expect("Error fetching user");
        let _user = match user_result.deserialize_current() {
            Ok(user) => user,
            Err(_) => return Err(TweetError::Unauthorized("".into())),
        };
        let token = _user.generate_token(&auth.password);
        if token.is_empty() {
            return Err(TweetError::Unauthorized("authentication failed".into()));
        }
        return Ok(token);
    }
}
