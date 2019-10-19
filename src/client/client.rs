use reqwest::Client as ReqClient;

// implement a client cache?

pub struct SfClient {
    inner: ReqClient,
}

impl SfClient {
    pub fn new() -> Self {
        Self{
            inner: ReqClient::new()
        }
    }
}

/*
use reqwest::Client;

pub async fn load_card() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let resp : List<Card> = client
        .get("https://api.scryfall.com/cards/search")
        .query(&[("q", "Ghalta"), ("pretty", "true")])
        .send()
        .await?
        .json()
        .await?;

    dbg!(resp);


    Ok(())
}*/