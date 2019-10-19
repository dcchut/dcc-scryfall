use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _multiverse_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .multiverse(409574)
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Strip Mine");

    Ok(())
}

#[test]
pub fn multiverse_request() {
    helper::block_on(_multiverse_request());
}
