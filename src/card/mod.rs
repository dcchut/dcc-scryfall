use crate::card::core::Core;
use crate::card::gameplay::Gameplay;
use crate::card::print::Print;
use serde::{Deserialize, Serialize};

pub mod core;
pub mod face;
pub mod gameplay;
pub mod legality;
pub mod price;
pub mod print;
pub mod related;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(flatten)]
    pub core: Core,

    #[serde(flatten)]
    pub gameplay: Gameplay,

    #[serde(flatten)]
    pub print: Print,
}