use dcc_scryfall::{SfClient, SfResult};

mod helper;

async fn _invalid_named_request() -> SfResult<()> {
    let client = SfClient::new();
    let invalid_card = client.card_named(false, "_some fake card name__").await;

    assert_eq!(invalid_card.is_err(), true);

    Ok(())
}

#[test]
pub fn invalid_named_request() {
    helper::block_on(_invalid_named_request());
}
