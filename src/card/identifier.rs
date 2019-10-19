use serde_json::json;

pub enum Identifier {
    /// Card with the specified Scryfall ID
    Id(String),

    /// Card with the specified `mtgo_id` or `mtgo_foil_id`
    MTGOId(usize),

    /// Card with the specified value among its `multiverse_ids`
    MultiverseId(usize),

    /// The newest edition of cards with the specified `oracle_id`
    OracleId(String),

    /// The preferred scans of cards with the specified `illustration_id`.
    IllustrationId(String),

    /// The newest edition of a card with the specified `name`.
    Name(String),

    /// The card matching the specified `name` and `set`
    NameAndSet(String, String),

    /// The card with the specified `collector_number` and `set`.
    CollectorNumberAndSet(String, String),
}

impl Identifier {
    pub(crate) fn as_value(&self) -> serde_json::Value {
        match self {
            Identifier::Id(id) => json!({ "id": id }),
            Identifier::MTGOId(mtgo_id) => json!({ "mtgo_id": mtgo_id }),
            Identifier::MultiverseId(multiverse_id) => json!({ "multiverse_id": multiverse_id }),
            Identifier::OracleId(oracle_id) => json!({ "oracle_id": oracle_id }),
            Identifier::IllustrationId(illustration_id) => {
                json!({ "illustration_id": illustration_id })
            }
            Identifier::Name(name) => json!({ "name": name }),
            Identifier::NameAndSet(name, set) => json!({
            "name": name,
            "set": set
            }),
            Identifier::CollectorNumberAndSet(collector_number, set) => json!({
            "collector_number": collector_number,
            "set": set
            }),
        }
    }
}
