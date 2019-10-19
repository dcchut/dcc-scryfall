use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _random_request() -> SfResult<()> {
    let client = SfClient::new();
    let random_card = client.random().await.expect("Unable to load random card");

    // Check that the ID field is populated
    assert!(random_card.core.id.len() > 0);

    Ok(())
}

#[test]
pub fn random_request() {
    helper::block_on(_random_request());
}
