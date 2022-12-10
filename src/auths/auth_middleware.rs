use super::utils::get_jwt_key;
use crate::errors::error::TweetError;
use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::extractors::AuthenticationError;
use jwt::{RegisteredClaims, VerifyWithKey};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token_string = credentials.token();

    let claims: Result<RegisteredClaims, TweetError> = token_string
        .verify_with_key(&get_jwt_key())
        .map_err(|_| TweetError::Unauthorized("Invalid token".into()));
    match claims {
        Ok(claim) => {
            req.extensions_mut().insert(claim);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
