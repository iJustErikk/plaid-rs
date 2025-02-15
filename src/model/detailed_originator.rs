
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetailedOriginator {
    pub company_name: String,
    pub transfer_diligence_status: String,
}
impl std::fmt::Display for DetailedOriginator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}