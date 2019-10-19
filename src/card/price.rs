use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
    pub tix: Option<String>,
}
