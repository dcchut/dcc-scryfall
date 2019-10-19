use dcc_scryfall::{SfClient, SfResult};

mod helper;

async fn _card_list_request() -> SfResult<()> {
    let client = SfClient::new();
    let card_list = client.card_search("Ghalta, Primal Hunger").await?;

    // Should only be 1 response
    assert_eq!(card_list.data.len(), 1);

    Ok(())
}

#[test]
pub fn card_list_request() {
    helper::block_on(_card_list_request());
}
