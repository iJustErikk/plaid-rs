
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMigrateAccountResponse {
    pub access_token: String,
    pub account_id: String,
    pub request_id: String,
}
impl std::fmt::Display for TransferMigrateAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}