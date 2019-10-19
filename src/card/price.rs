use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub usd: String,
    pub usd_foil: String,
    pub eur: String,
    pub tix: String,
}
