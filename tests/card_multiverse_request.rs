use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn multiverse_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_multiverse(409574)
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Strip Mine");

    Ok(())
}