use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn fuzzy_named_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_named(true, "jac bele")
        .await
        .expect("Unable to load card");

    assert_eq!(card.core.id, "97956927-135f-47b0-8aef-cc63889eab5a");
    assert_eq!(card.gameplay.name, "Jace Beleren");

    Ok(())
}
