#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let authorization_id = "your authorization id";
    let description = "your description";
    let response = client
        .transfer_create(authorization_id, description)
        .access_token("your access token")
        .account_id("your account id")
        .ach_class("your ach class")
        .amount("your amount")
        .idempotency_key("your idempotency key")
        .iso_currency_code("your iso currency code")
        .metadata(TransferMetadata {})
        .network("your network")
        .origination_account_id("your origination account id")
        .payment_profile_token("your payment profile token")
        .type_("your type")
        .user(TransferUserInRequestDeprecated {
            address: Some(TransferUserAddressInRequest {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}