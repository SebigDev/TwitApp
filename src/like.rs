use std::str::FromStr;

use super::schema::likes;
use crate::dbconn::{DbPooledConnection, DbPool};
use crate::response::Response;
use actix_web::web::{Data, block};
use actix_web::{post, delete};
use actix_web::{get, web::Path, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Likes = Response<Like>;
pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn to_like_db(&self, tweet_id: Uuid) -> LikeDB {
        LikeDB {
            id: Uuid::from_str(&self.id).unwrap(),
            created_at: self.created_at.naive_utc(),
            tweet_id,
        }
    }
}

//DB Tables

#[derive(Queryable, Insertable)]
#[diesel(table_name = likes)]
pub struct LikeDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl LikeDB {
    pub fn to_like(&self) -> Like {
        Like {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }
}

pub fn all_likes(_tweet_id: Uuid, conn: &mut DbPooledConnection) -> Result<Likes, Error> {
    use crate::schema::likes::dsl::*;

    let _likes: Vec<LikeDB> = match likes
        .filter(tweet_id.eq(_tweet_id))
        .order(created_at.desc())
        .load::<LikeDB>(&mut *conn)
    {
        Ok(lks) => lks,
        Err(_) => vec![],
    };
    Ok(Likes {
        results: _likes.into_iter().map(|l| l.to_like()).collect(),
    })
}

pub fn create_like(_tweet_id: Uuid, conn: &mut DbPooledConnection) -> Result<Like, Error> {
    use crate::schema::likes::dsl::*;

    let like = Like::new();
    let _ = diesel::insert_into(likes)
        .values(like.to_like_db(_tweet_id))
        .execute(&mut *conn);
    Ok(like)
}

pub fn delete_like(_tweet_id: Uuid, conn: &mut DbPooledConnection) -> Result<(), Error> {
    use crate::schema::likes::dsl::*;

    let _likes = all_likes(_tweet_id, conn);
    let like = match &_likes {
        Ok(_likes) if !_likes.results.is_empty() => _likes.results.first(),
        _ => None,
    };

    if like.is_none() {
        return Ok(());
    }

    let like_id = Uuid::from_str(like.unwrap().id.as_str()).unwrap();
    let res = diesel::delete(likes.filter(id.eq(like_id))).execute(&mut *conn);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>, pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");

    let likes =
        block(move || all_likes(Uuid::from_str(path.0.as_str()).unwrap(), &mut conn)).await;

    match likes {
        Ok(likes) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(likes.unwrap()),
        _ => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(Like::new()),
    }
}

/// add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>, pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");

    let like =
        block(move || create_like(Uuid::from_str(path.0.as_str()).unwrap(), &mut conn)).await;

    match like {
        Ok(like) => HttpResponse::Ok().content_type(APPLICATION_JSON).json(like.unwrap()),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>, pool: Data<DbPool>) -> HttpResponse {
    // in any case return status 204
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");

    let _ = block(move || delete_like(Uuid::from_str(path.0.as_str()).unwrap(), &mut conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
