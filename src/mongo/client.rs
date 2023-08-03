use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn create() -> Client {
    let client_uri =
        env::var("MONGO_DB_URI").expect("You must set the MONGO_DB_URI environment var!");

    let options = ClientOptions::parse(&client_uri).await.unwrap();

    Client::with_options(options).unwrap()
}
