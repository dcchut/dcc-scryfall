use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn set_code_request() -> SfResult<()> {
    let client = SfClient::new();
    let set = client
        .set_code(String::from("mmq"))
        .await
        .expect("Unable to load set");

    assert_eq!(set.name, "Mercadian Masques");

    Ok(())
}
