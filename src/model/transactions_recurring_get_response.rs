
use serde::{Serialize, Deserialize};
use super::TransactionStream;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    pub inflow_streams: Vec<TransactionStream>,
    pub outflow_streams: Vec<TransactionStream>,
    pub request_id: String,
    pub updated_datetime: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for TransactionsRecurringGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}