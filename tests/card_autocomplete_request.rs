use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn autocomplete() -> SfResult<()> {
    let client = SfClient::new();
    let completions = client
        .card_autocomplete("thal")
        .await
        .expect("Unable to load autocomplete");

    // More Thalia's may be printed in the future.
    assert!(completions.data.len() >= 20);

    Ok(())
}
