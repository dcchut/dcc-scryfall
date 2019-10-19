use crate::card::face::Face;
use crate::card::legality::Legality;
use crate::card::related::Related;
use crate::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gameplay {
    pub all_parts: Option<Vec<Related>>,
    pub card_faces: Option<Vec<Face>>,
    pub cmc: f32,
    pub colors: Option<Vec<Color>>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub edhrec_rank: Option<usize>,
    pub foil: bool,
    pub hand_modifier: Option<String>,
    pub layout: String,
    pub legalities: Legality,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub power: Option<String>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,
}
