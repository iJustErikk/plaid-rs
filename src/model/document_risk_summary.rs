
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<f64>,
}
impl std::fmt::Display for DocumentRiskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}