#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let auditor_id = "your auditor id";
    let response = client
        .asset_report_audit_copy_create(asset_report_token, auditor_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
