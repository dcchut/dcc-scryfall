use dcc_scryfall::{card::Identifier, SfClient, SfResult};

mod helper;

async fn _identifiers_request() -> SfResult<()> {
    let client = SfClient::new();

    let id1 = Identifier::Id(String::from("683a5707-cddb-494d-9b41-51b4584ded69"));
    let id2 = Identifier::Name(String::from("Ancient Tomb"));
    let id3 = Identifier::CollectorNumberAndSet(String::from("150"), String::from("mrd"));

    let identifiers = vec![id1, id2, id3];

    let response = client
        .card_collections(identifiers)
        .await
        .expect("Unable to load collections");

    // Should be 3 cards in the response
    assert_eq!(response.data.len(), 3);

    assert_eq!(response.data[0].gameplay.name, "Lodestone Golem");
    assert_eq!(response.data[1].gameplay.name, "Ancient Tomb");
    assert_eq!(response.data[2].gameplay.name, "Chalice of the Void");

    Ok(())
}

#[test]
pub fn identifiers_request() {
    helper::block_on(_identifiers_request());
}
