use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn arena_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client.card_arena(67330).await.expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Yargle, Glutton of Urborg");

    Ok(())
}
