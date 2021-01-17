use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn set_tcgplayer_request() -> SfResult<()> {
    let client = SfClient::new();
    let set = client
        .set_tcgplayer(1909)
        .await
        .expect("Unable to load set");

    assert_eq!(set.name, "Amonkhet Invocations");

    Ok(())
}
