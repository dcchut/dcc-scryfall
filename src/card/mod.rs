pub use crate::card::core::Core;
pub use crate::card::face::Face;
pub use crate::card::gameplay::Gameplay;
pub use crate::card::legality::Legality;
pub use crate::card::price::Price;
pub use crate::card::print::Print;
pub use crate::card::related::Related;
pub use crate::card::identifier::Identifier;

use serde::{Deserialize, Serialize};

mod core;
mod face;
mod gameplay;
mod identifier;
mod legality;
mod price;
mod print;
mod related;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(flatten)]
    pub core: Core,

    #[serde(flatten)]
    pub gameplay: Gameplay,

    #[serde(flatten)]
    pub print: Print,
}
