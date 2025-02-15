
use serde::{Serialize, Deserialize};
use super::TransferIntentCreate;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreateResponse {
    pub request_id: String,
    pub transfer_intent: TransferIntentCreate,
}
impl std::fmt::Display for TransferIntentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}