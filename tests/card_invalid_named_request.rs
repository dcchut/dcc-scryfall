use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn invalid_named_request() -> SfResult<()> {
    let client = SfClient::new();
    let invalid_card = client.card_named(false, "_some fake card name__").await;

    assert_eq!(invalid_card.is_err(), true);

    Ok(())
}
