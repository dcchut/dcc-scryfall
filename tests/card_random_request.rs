use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn random_request() -> SfResult<()> {
    let client = SfClient::new();
    let random_card = client
        .card_random()
        .await
        .expect("Unable to load random card");

    // Check that the ID field is populated
    assert!(random_card.core.id.len() > 0);

    Ok(())
}
