use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _set_collector_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_set_and_collector_number(String::from("xln"), 96)
        .await
        .expect("Unable to load set and collector number");

    assert_eq!(card.gameplay.name, "Costly Plunder");

    Ok(())
}

#[test]
pub fn set_collector_request() {
    helper::block_on(_set_collector_request());
}
