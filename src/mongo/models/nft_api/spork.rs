use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::options::FindOneOptions;
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Spork {
    pub _id: ObjectId,
    pub version: i32,
    pub start_height: i64,
    pub access_url: String,
}

impl Spork {
    pub fn new(start_height: i64, access_url: String, version: i32) -> Self {
        Spork {
            _id: ObjectId::new(),
            version,
            start_height,
            access_url,
        }
    }

    pub async fn create(
        client: &Client,
        start_height: i64,
        access_url: String,
        version: i32,
    ) -> Self {
        let sp = Spork::new(start_height, access_url, version);
        sp.clone().save(client).await;
        sp
    }

    pub async fn save(self, client: &Client) {
        Spork::get_collection(client).insert_one(self, None).await;
    }

    pub async fn get(client: &Client, height: i64) -> Option<Spork> {
        let s_col = Spork::get_collection(client);
        s_col
            .find_one(
                mongo_doc! { "start_height": { "$lt": height } },
                FindOneOptions::builder()
                    .sort(mongo_doc! { "start_height": -1 })
                    .build(),
            )
            .await
            .unwrap()
    }
}
