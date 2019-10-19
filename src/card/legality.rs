use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legality {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub brawl: String,
    pub duel: String,
    pub oldschool: String,
}
