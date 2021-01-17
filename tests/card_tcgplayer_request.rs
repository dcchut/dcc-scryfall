use dcc_scryfall::{SfClient, SfResult};

#[tokio::test]
async fn tcgplayer_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_tcgplayer(162145)
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Rona, Disciple of Gix");

    Ok(())
}
