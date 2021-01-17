use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn mtgo_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client.card_mtgo(54957).await.expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Ghost Quarter");

    Ok(())
}
