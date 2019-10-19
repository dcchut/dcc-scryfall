use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _mtgo_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client.card_mtgo(54957).await.expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Ghost Quarter");

    Ok(())
}

#[test]
pub fn mtgo_request() {
    helper::block_on(_mtgo_request());
}
