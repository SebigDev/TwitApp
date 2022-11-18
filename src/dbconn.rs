use dotenv::dotenv;
use mongodb::{Client, Collection, options::ClientOptions};
use std::env;

#[derive(Clone)]
pub struct MongoPool<T> {
    pub collection: Collection<T>,
}

impl<T> MongoPool<T> {
    pub async fn connect() -> Self {
        dotenv().ok();
        let database_url =  env::var("MONGODB_URL").expect("MONGODB_URL is required");

        let client_options = ClientOptions::parse(&database_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        
        let db_name = env::var("DATABASE_NAME").expect("Database name must be set");
        let database = client.database(&db_name);

        let collection_name = std::any::type_name::<T>();
        println!("Collection Name {}", collection_name);
        let collection: Collection<T> = database.collection(collection_name);

        MongoPool { collection }
    }
}
