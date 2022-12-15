use crate::{
    auths::auth::AuthData, dtos::dto::UserDto, errors::error::TweetError, model::auth_model::User,
};
use bson::doc;
use mongodb::{Collection, Cursor};

pub struct UserRepo<User> {
    pub collection: Collection<User>,
}

impl UserRepo<User> {
    pub async fn register(&self, user: User) -> Result<UserDto, TweetError> {
        let user_result = self.get_user_by_email(&user.email).await;
        match user_result {
            Ok(mut resp) => {
                while resp.advance().await.unwrap() {
                    return Err(TweetError::BadRequest(format!(
                        "User with {} already exists",
                        user.email
                    )));
                }
            }
            Err(_) => return Err(TweetError::InternalServerError),
        };
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
        let user_result = self.get_user_by_email(&auth.email).await;
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
            Err(_) => return Err(TweetError::InternalServerError),
        };
    }

    /// Get user by email address
    async fn get_user_by_email(&self, email: &str) -> Result<Cursor<User>, TweetError> {
        let filter = doc! {"email": &email};
        let user_result = self
            .collection
            .find(filter, None)
            .await
            .map_err(|_| TweetError::InternalServerError);
        return user_result;
    }
}
