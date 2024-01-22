use tokio::main;

use celestia_rpc::{BlobClient, HeaderClient, Client};
use celestia_types::{Blob, nmt::Namespace};
use celestia_types::blob::SubmitOptions;

use thiserror::Error;

struct RollupState {
    sync_height: u64,
    block_height: u64,
    namespace: Namespace,
}

#[derive(Error, Debug)]
pub enum WhimsifallError {
    #[error("Deserialization error")]
    DeserializationError,
}

struct Block {
    height: u64,
    gas_used: u64,
}

impl Block {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.gas_used.to_be_bytes());
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, WhimsifallError> {
        let height = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let gas_used = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        Ok(Block {
            height,
            gas_used,
        })
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("CELESTIA_NODE_AUTH_TOKEN").expect("Token not provided");
    let start_height = std::env::var("START_HEIGHT").expect("Start height not provided");
    let start_height: u64 = start_height.parse().expect("Could not parse start height");
    let namespace = Namespace::new(0, 
        &std::env::var("ROLLUP_NAMESPACE").expect("Rollup namespace not provided").into_bytes()
    ).expect("Invalid namespace");
    let client = Client::new("ws://localhost:26658", Some(&token))
        .await
        .expect("Failed creating rpc client");
    let network_head = client.header_network_head()
        .await
        .expect("could not get network head");
    let mut rollup = RollupState {
        sync_height: start_height,
        block_height: 1,
        namespace: namespace,
    };
    sync_da_height(&mut rollup, &client, start_height).await;
}

async fn sync_da_height(rollup_state: &mut RollupState, client: &Client, da_height: u64) {
    let blobs = client.blob_get_all(da_height, &[rollup_state.namespace])
        .await
        .expect("Could not get blobs");
    for blob in &blobs {
        let block = Block::deserialize(&blob.data);
    }
    println!("got {} blobs", blobs.len());
}

/*async fn submit_blob() {
    // create a client to the celestia node
    let token = std::env::var("CELESTIA_NODE_AUTH_TOKEN").expect("Token not provided");
    let client = Client::new("ws://localhost:26658", Some(&token))
        .await
        .expect("Failed creating rpc client");

    // create a blob that you want to submit
    let my_namespace = Namespace::new_v0(&[1, 2, 3, 4, 5]).expect("Invalid namespace");
    let blob = Blob::new(my_namespace, b"some data to store on blockchain".to_vec())
        .expect("Failed to create a blob");

    // submit it
    client.blob_submit(&[blob], SubmitOptions::default())
        .await
        .expect("Failed submitting the blob");
}*/