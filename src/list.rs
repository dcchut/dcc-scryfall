use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub next_page: Option<String>,
    pub total_cards: Option<usize>,
    pub warnings: Option<Vec<String>>,
}
