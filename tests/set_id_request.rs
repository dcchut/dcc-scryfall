use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn set_id_request() -> SfResult<()> {
    let client = SfClient::new();
    let set = client
        .set_id(String::from("2ec77b94-6d47-4891-a480-5d0b4e5c9372"))
        .await
        .expect("Unable to load set");

    assert_eq!(set.name, "Ultimate Masters");

    Ok(())
}
