use actix_web::{
    post,
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};

use crate::{
    auths::auth::{AuthData, CreateUser},
    model::auth_model::User,
    repo::user_repo::UserRepo,
};

#[post("/api/v1/user/register")]
pub async fn register(db: Data<UserRepo<User>>, new_user: Json<CreateUser>) -> impl Responder {
    let data: CreateUser = new_user.into_inner();
    let user = User::new(&data.email, &data.password);
    let result = db.register(user).await;
    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/v1/user/login")]
pub async fn login(db: Data<UserRepo<User>>, auth: Json<AuthData>) -> impl Responder {
    let user: AuthData = auth.into_inner();
    let result = db.valid_user(user).await;
    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}

#[post("/user/logout")]
pub async fn signout(req: HttpRequest) -> impl Responder {
    let mut request = req.extensions_mut();
    request.clear();
    HttpResponse::Ok().json({})
}
