use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Setting {
    pub _id: ObjectId,
    pub latest_requested_block: u64,
}

impl Setting {
    pub fn new() -> Self {
        Setting {
            _id: ObjectId::new(),
            latest_requested_block: 0,
        }
    }
    pub async fn save(self, client: &Client) {
        Setting::get_collection(client).insert_one(self, None).await;
    }
    pub async fn get(client: &Client) -> Setting {
        let s_col = Setting::get_collection(client);
        let settings: Vec<Setting> = s_col
            .find(None, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        settings.first().unwrap().clone()
    }

    pub async fn update(&self, client: &Client, session: Option<&mut ClientSession>) {
        let s_col = Setting::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        let doc_update = mongo_doc! {
            "$set" : {
                "latest_requested_block": self.latest_requested_block as i64
            }
        };
        let _ = match session {
            Some(s) => s_col.update_one_with_session(q, doc_update, None, s).await,
            _ => s_col.update_one(q, doc_update, None).await,
        };
    }
}
