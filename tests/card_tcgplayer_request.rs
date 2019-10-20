use dcc_scryfall::{SfClient, SfResult};

mod helper;

async fn _tcgplayer_request() -> SfResult<()> {
    let client = SfClient::new();
    let card = client
        .card_tcgplayer(162145)
        .await
        .expect("Unable to load card");

    assert_eq!(card.gameplay.name, "Rona, Disciple of Gix");

    Ok(())
}

#[test]
pub fn tcgplayer_request() {
    helper::block_on(_tcgplayer_request());
}
