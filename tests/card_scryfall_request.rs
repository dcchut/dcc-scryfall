use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn scryfall_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_id(String::from("c8b4d10d-ecf4-4dad-89d3-12333b522358"))
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Dusk // Dawn");
    assert_eq!(card.gameplay.card_faces.unwrap().len(), 2);

    Ok(())
}
