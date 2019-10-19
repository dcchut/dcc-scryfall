use dcc_scryfall::Card;

#[test]
pub fn deserialize_single_card() {
    let json = r#"{
  "object": "card",
  "id": "97956927-135f-47b0-8aef-cc63889eab5a",
  "oracle_id": "cc2b6e58-2d6f-495e-95f3-9496105c4cba",
  "multiverse_ids": [
    393851
  ],
  "tcgplayer_id": 93796,
  "name": "Jace Beleren",
  "lang": "en",
  "released_at": "2014-12-05",
  "uri": "https://api.scryfall.com/cards/97956927-135f-47b0-8aef-cc63889eab5a",
  "scryfall_uri": "https://scryfall.com/card/jvc/1/jace-beleren?utm_source=api",
  "layout": "normal",
  "highres_image": true,
  "image_uris": {
    "small": "https://img.scryfall.com/cards/small/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.jpg?1562378464",
    "normal": "https://img.scryfall.com/cards/normal/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.jpg?1562378464",
    "large": "https://img.scryfall.com/cards/large/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.jpg?1562378464",
    "png": "https://img.scryfall.com/cards/png/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.png?1562378464",
    "art_crop": "https://img.scryfall.com/cards/art_crop/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.jpg?1562378464",
    "border_crop": "https://img.scryfall.com/cards/border_crop/front/9/7/97956927-135f-47b0-8aef-cc63889eab5a.jpg?1562378464"
  },
  "mana_cost": "{1}{U}{U}",
  "cmc": 3.0,
  "type_line": "Legendary Planeswalker — Jace",
  "oracle_text": "+2: Each player draws a card.\n−1: Target player draws a card.\n−10: Target player puts the top twenty cards of their library into their graveyard.",
  "loyalty": "3",
  "colors": [
    "U"
  ],
  "color_identity": [
    "U"
  ],
  "legalities": {
    "standard": "not_legal",
    "future": "not_legal",
    "historic": "not_legal",
    "modern": "legal",
    "legacy": "legal",
    "pauper": "not_legal",
    "vintage": "legal",
    "penny": "not_legal",
    "commander": "legal",
    "brawl": "not_legal",
    "duel": "legal",
    "oldschool": "not_legal"
  },
  "games": [
    "paper"
  ],
  "reserved": false,
  "foil": true,
  "nonfoil": false,
  "oversized": false,
  "promo": false,
  "reprint": true,
  "variation": false,
  "set": "jvc",
  "set_name": "Duel Decks Anthology: Jace vs. Chandra",
  "set_type": "duel_deck",
  "set_uri": "https://api.scryfall.com/sets/37832684-2fe0-4b06-842b-eec06e5a17cb",
  "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Ajvc&unique=prints",
  "scryfall_set_uri": "https://scryfall.com/sets/jvc?utm_source=api",
  "rulings_uri": "https://api.scryfall.com/cards/97956927-135f-47b0-8aef-cc63889eab5a/rulings",
  "prints_search_uri": "https://api.scryfall.com/cards/search?order=released&q=oracleid%3Acc2b6e58-2d6f-495e-95f3-9496105c4cba&unique=prints",
  "collector_number": "1",
  "digital": false,
  "rarity": "mythic",
  "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
  "artist": "Kev Walker",
  "artist_ids": [
    "f366a0ee-a0cd-466d-ba6a-90058c7a31a6"
  ],
  "illustration_id": "00146337-02df-4b1e-976b-8d64af966dd8",
  "border_color": "black",
  "frame": "2015",
  "full_art": false,
  "textless": false,
  "booster": false,
  "story_spotlight": false,
  "edhrec_rank": 775,
  "prices": {
    "usd": null,
    "usd_foil": "4.41",
    "eur": null,
    "tix": null
  },
  "related_uris": {
    "gatherer": "https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=393851",
    "tcgplayer_decks": "https://decks.tcgplayer.com/magic/deck/search?contains=Jace+Beleren&page=1&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
    "edhrec": "https://edhrec.com/route/?cc=Jace+Beleren",
    "mtgtop8": "https://mtgtop8.com/search?MD_check=1&SB_check=1&cards=Jace+Beleren"
  },
  "purchase_uris": {
    "tcgplayer": "https://shop.tcgplayer.com/product/productsearch?id=93796&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
    "cardmarket": "https://www.cardmarket.com/en/Magic/Products/Search?referrer=scryfall&searchString=Jace+Beleren&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall",
    "cardhoarder": "https://www.cardhoarder.com/cards?affiliate_id=scryfall&data%5Bsearch%5D=Jace+Beleren&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall"
  }
}"#;

    let card: Card = serde_json::from_str(json).expect("Unable to parse card");

    assert_eq!(card.gameplay.name, "Jace Beleren");
}
