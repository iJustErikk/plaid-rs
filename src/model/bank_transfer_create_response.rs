
use serde::{Serialize, Deserialize};
use super::BankTransfer;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCreateResponse {
    pub bank_transfer: BankTransfer,
    pub request_id: String,
}
impl std::fmt::Display for BankTransferCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}