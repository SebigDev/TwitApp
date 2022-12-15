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

    pub async fn valid_user(&self, auth: &AuthData) -> Result<String, TweetError> {
        let filter = doc! {"email": &auth.email};
        let user_result = self
            .collection
            .find(filter, None)
            .await
            .map_err(|_| TweetError::InternalServerError);

        match user_result {
            Ok(mut user) => {
                while user.advance().await.unwrap() {
                    let _user = match user.deserialize_current() {
                        Ok(user) => user,
                        Err(_) => {
                            return Err(TweetError::Unauthorized("User data is not valid".into()))
                        }
                    };
                    let token_option = _user.generate_token(&auth.password);
                    let token = match token_option {
                        Some(_token) => _token,
                        None => { return Err(TweetError::Unauthorized("authentication failed, please check that email and password are correct".into()))
                        }
                    };
                    return Ok(token);
                }
                return Err(TweetError::Unauthorized(
                    format!("No user with {} found.", &auth.email).into(),
                ));
            }
            Err(_) => return Err(TweetError::Unauthorized("User does not exist".into())),
        };
    }
}
