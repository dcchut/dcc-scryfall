use dcc_scryfall::{SfClient, SfResult};

mod helper;

async fn _set_tcgplayer_request() -> SfResult<()> {
    let client = SfClient::new();
    let set = client
        .set_tcgplayer(1909)
        .await
        .expect("Unable to load set");

    assert_eq!(set.name, "Amonkhet Invocations");

    Ok(())
}

#[test]
pub fn set_tcgplayer_request() {
    helper::block_on(_set_tcgplayer_request());
}
