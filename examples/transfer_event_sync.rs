#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let after_id = 1;
    let response = client.transfer_event_sync(after_id).count(1).await.unwrap();
    println!("{:#?}", response);
}