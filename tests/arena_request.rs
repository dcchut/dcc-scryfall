use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _arena_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client.arena(67330)
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Yargle, Glutton of Urborg");

    Ok(())
}

#[test]
pub fn arena_request() {
    helper::block_on(_arena_request());
}
