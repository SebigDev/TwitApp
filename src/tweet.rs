use crate::dbconn::{DbPool, DbPooledConnection};
use crate::like::all_likes;
use crate::response::Response;
use crate::{like::Like, schema::tweets};
use actix_web::web::{block, Data, Path};
use actix_web::{delete, get, post, web::Json, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use std::str::FromStr;

pub type Tweets = Response<Tweet>;
pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message: message.to_string(),
            likes: vec![],
        }
    }

    pub fn to_tweet_db(&self) -> TweetDB {
        TweetDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message: self.message.clone(),
        }
    }

    pub fn add_likes(&self, likes: Vec<Like>) -> Self {
        Self {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            message: self.message.clone(),
            likes,
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = tweets)]
pub struct TweetDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub message: String,
}

impl TweetDB {
    pub fn to_tweet(&self) -> Tweet {
        Tweet {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            message: self.message.clone(),
            likes: vec![],
        }
    }
}

pub trait TweetActions {
    fn tweet(&self) -> Option<Tweet>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    message: Option<String>,
}

impl TweetActions for TweetRequest {
    fn tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message)),
            None => None,
        }
    }
}

fn list_tweets(all_tweets: i64, conn: &mut DbPooledConnection) -> Result<Tweets, Error> {
    use crate::schema::tweets::dsl::*;

    let _tweets: Vec<TweetDB> = match tweets
        .order(created_at.desc())
        .limit(all_tweets)
        .load::<TweetDB>(&mut *conn)
    {
        Ok(twts) => twts,
        Err(_) => vec![],
    };
    Ok(Tweets {
        results: _tweets
            .into_iter()
            .map(|t| t.to_tweet())
            .collect::<Vec<Tweet>>(),
    })
}

fn fiind_tweet(_id: Uuid, conn: &mut DbPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;
    let resp = tweets.filter(id.eq(_id)).load::<TweetDB>(&mut *conn);
    match resp {
        Ok(tweets_db) => match tweets_db.first() {
            Some(tweet_db) => Ok(tweet_db.to_tweet()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

fn create_tweet(tweet: Tweet, conn: &mut DbPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let tweet_db = tweet.to_tweet_db();
    let _ = diesel::insert_into(tweets)
        .values(&tweet_db)
        .execute(&mut *conn);
    Ok(tweet_db.to_tweet())
}

fn delete_tweet(_id: Uuid, conn: &mut DbPooledConnection) -> Result<(), Error> {
    use crate::schema::tweets::dsl::*;

    let del = diesel::delete(tweets.filter(id.eq(_id))).execute(&mut *conn);

    match del {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[post("/tweet")]
pub async fn create(tweet_request: Json<TweetRequest>, pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");

    let tweet = block(move || create_tweet(tweet_request.tweet().unwrap(), &mut conn)).await;

    match tweet {
        Ok(tweet) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet.unwrap()),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

#[get("/tweet/{id}")]
pub async fn get(path: Path<(String,)>, pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let tweet =
        block(move || fiind_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &mut conn)).await;
    match tweet {
        Ok(tweet) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet.unwrap()),
        _ => HttpResponse::NotFound().await.unwrap(),
    }
}

#[get("/tweets")]
pub async fn list(pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let tweets = block(move || list_tweets(50, &mut conn)).await.unwrap();

    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let mut _tweets = tweets.unwrap();
    let tweets_with_likes = Tweets {
        results: _tweets
            .results
            .iter_mut()
            .map(|t| {
                let _likes = all_likes(Uuid::from_str(t.id.as_str()).unwrap(), &mut conn).unwrap();
                t.add_likes(_likes.results)
            })
            .collect::<Vec<Tweet>>(),
    };
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets_with_likes)
}

#[delete("/tweet/{id}")]
pub async fn delete(path: Path<(String,)>, pool: Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let _ = block(move || delete_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &mut conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
