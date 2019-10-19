use dcc_scryfall::{SfClient, SfResult};
mod helper;

async fn _autocomplete_request() -> SfResult<()> {
    let client = SfClient::new();
    let completions = client
        .card_autocomplete("thal")
        .await
        .expect("Unable to load autocomplete");

    assert_eq!(completions.data.len(), 20);

    Ok(())
}

#[test]
pub fn autocomplete_request() {
    helper::block_on(_autocomplete_request());
}
