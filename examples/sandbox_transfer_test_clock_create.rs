#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let virtual_time = chrono::Utc::now();
    let response = client
        .sandbox_transfer_test_clock_create(virtual_time)
        .await
        .unwrap();
    println!("{:#?}", response);
}