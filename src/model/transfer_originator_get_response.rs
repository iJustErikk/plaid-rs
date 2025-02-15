
use serde::{Serialize, Deserialize};
use super::DetailedOriginator;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorGetResponse {
    pub originator: DetailedOriginator,
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}