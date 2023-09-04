use flow_helpers::flow_rs::access::{GetTransactionRequest, PingRequest, SendTransactionRequest};
use flow_helpers::flow_rs::cadence_json::ValueOwned;
use flow_helpers::flow_rs::proto::{
    access::{
        access_api_client::AccessApiClient, BlockResponse, GetBlockByHeightRequest,
        GetCollectionByIdRequest, GetLatestBlockRequest, GetTransactionsByBlockIdRequest,
    },
    execution::GetTransactionResultRequest,
};
use flow_helpers::flow_rs::FlowNetwork;
use flow_helpers::mongo::{self, models::Setting};
use std::str;

#[tokio::main]
async fn main() {
    let mut client = FlowNetwork::Mainnet.get_flow_client().await;
    let m_client = mongo::client::create().await;
    let mut latest_block_height = 59430117;
    let mut s = Setting::get(&m_client).await;
    println!("{:?}", s);
    s.latest_requested_block = 100;
    s.update(&m_client, None).await;
    // loop {
    //     let r = match client
    //         .get_block_by_height(GetBlockByHeightRequest {
    //             height: latest_block_height,
    //             full_block_response: true,
    //         })
    //         .await
    //     {
    //         Ok(x) => x.into_inner(),
    //         Err(e) => {
    //             println!("{:?}", e);
    //             continue;
    //         }
    //     };
    //     let block = r.clone().block.unwrap();

    //     latest_block_height += 1;

    //     println!(
    //         "block height: {}, id: {:?}",
    //         block.height,
    //         hex::encode(block.id.clone())
    //     );

    //     let k = block.id.clone();
    //     for collection in block.collection_guarantees {
    //         let col = match client
    //             .get_collection_by_id(GetCollectionByIdRequest {
    //                 id: collection.collection_id,
    //             })
    //             .await
    //         {
    //             Ok(x) => x.into_inner(),
    //             Err(e) => {
    //                 println!("{:?}", e);
    //                 continue;
    //             }
    //         };

    //         for tr_id in col.collection.clone().unwrap().clone().transaction_ids {
    //             let tran = match client
    //                 .get_transaction_result(GetTransactionRequest {
    //                     id: tr_id,
    //                     block_id: k.clone(),
    //                     collection_id: col.collection.clone().unwrap().id.clone(),
    //                 })
    //                 .await
    //             {
    //                 Ok(x) => x.into_inner(),
    //                 Err(e) => {
    //                     println!("{:?}", e);
    //                     continue;
    //                 }
    //             };
    //             for event in tran.events {
    //                 if let ValueOwned::Event(event_payload) =
    //                     serde_json::from_slice(&event.payload).unwrap()
    //                 {
    //                     println!("{:?}", event_payload.id);
    //                 }
    //             }
    //         }
    //     }
    // }
}
