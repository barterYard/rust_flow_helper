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
use flow_helpers::mongo::{self, models::Spork};
use std::str;
const THRESOLD: [(i64, &str); 23] = [
    (7601063, "http://access-001.mainnet1.nodes.onflow.org:9000"),
    (8742959, "http://access-001.mainnet2.nodes.onflow.org:9000"),
    (9737133, "http://access-001.mainnet3.nodes.onflow.org:9000"),
    (9992020, "http://access-001.mainnet4.nodes.onflow.org:9000"),
    (12020337, "http://access-001.mainnet5.nodes.onflow.org:9000"),
    (12609237, "http://access-001.mainnet6.nodes.onflow.org:9000"),
    (13404174, "http://access-001.mainnet7.nodes.onflow.org:9000"),
    (13950742, "http://access-001.mainnet8.nodes.onflow.org:9000"),
    (14892104, "http://access-001.mainnet9.nodes.onflow.org:9000"),
    (
        15791891,
        "http://access-001.mainnet10.nodes.onflow.org:9000",
    ),
    (
        16755602,
        "http://access-001.mainnet11.nodes.onflow.org:9000",
    ),
    (
        17544523,
        "http://access-001.mainnet12.nodes.onflow.org:9000",
    ),
    (
        18587478,
        "http://access-001.mainnet13.nodes.onflow.org:9000",
    ),
    (
        19050753,
        "http://access-001.mainnet14.nodes.onflow.org:9000",
    ),
    (
        21291692,
        "http://access-001.mainnet15.nodes.onflow.org:9000",
    ),
    (
        23830813,
        "http://access-001.mainnet16.nodes.onflow.org:9000",
    ),
    (
        27341470,
        "http://access-001.mainnet17.nodes.onflow.org:9000",
    ),
    (
        31735955,
        "http://access-001.mainnet18.nodes.onflow.org:9000",
    ),
    (
        35858811,
        "http://access-001.mainnet19.nodes.onflow.org:9000",
    ),
    (
        40171634,
        "http://access-001.mainnet20.nodes.onflow.org:9000",
    ),
    (
        44950207,
        "http://access-001.mainnet21.nodes.onflow.org:9000",
    ),
    (
        47169687,
        "http://access-001.mainnet22.nodes.onflow.org:9000",
    ),
    (55114467, "http://access.mainnet.nodes.onflow.org:9000"),
];

#[tokio::main]
async fn main() {
    let m_client = mongo::client::create().await;
    // for (index, sp) in THRESOLD.iter().enumerate() {
    //     Spork::create(
    //         &m_client,
    //         sp.0,
    //         sp.1.to_string(),
    //         (index + 1).try_into().unwrap(),
    //     )
    //     .await;
    // }
    let sp = Spork::get(&m_client, 12020512).await.unwrap();
    println!("{:?}", sp);
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
