use dcc_scryfall::{Card, List};

#[test]
pub fn deserialize_card_list() {
    let json = r#"{
  "object": "list",
  "total_cards": 1,
  "has_more": false,
  "data": [
    {
      "object": "card",
      "id": "0104b5b3-9376-4ad7-9a77-3e564e9c42e6",
      "oracle_id": "b0b6be0c-41cf-4757-9f0e-87227b6ba6b3",
      "multiverse_ids": [
        439787
      ],
      "mtgo_id": 66713,
      "arena_id": 66877,
      "tcgplayer_id": 155579,
      "name": "Ghalta, Primal Hunger",
      "lang": "en",
      "released_at": "2018-01-19",
      "uri": "https://api.scryfall.com/cards/0104b5b3-9376-4ad7-9a77-3e564e9c42e6",
      "scryfall_uri": "https://scryfall.com/card/rix/130/ghalta-primal-hunger?utm_source=api",
      "layout": "normal",
      "highres_image": true,
      "image_uris": {
        "small": "https://img.scryfall.com/cards/small/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.jpg?1555040636",
        "normal": "https://img.scryfall.com/cards/normal/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.jpg?1555040636",
        "large": "https://img.scryfall.com/cards/large/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.jpg?1555040636",
        "png": "https://img.scryfall.com/cards/png/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.png?1555040636",
        "art_crop": "https://img.scryfall.com/cards/art_crop/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.jpg?1555040636",
        "border_crop": "https://img.scryfall.com/cards/border_crop/front/0/1/0104b5b3-9376-4ad7-9a77-3e564e9c42e6.jpg?1555040636"
      },
      "mana_cost": "{10}{G}{G}",
      "cmc": 12.0,
      "type_line": "Legendary Creature — Elder Dinosaur",
      "oracle_text": "This spell costs {X} less to cast, where X is the total power of creatures you control.\nTrample",
      "power": "12",
      "toughness": "12",
      "colors": [
        "G"
      ],
      "color_identity": [
        "G"
      ],
      "legalities": {
        "standard": "not_legal",
        "future": "not_legal",
        "historic": "legal",
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
        "arena",
        "mtgo",
        "paper"
      ],
      "reserved": false,
      "foil": true,
      "nonfoil": true,
      "oversized": false,
      "promo": false,
      "reprint": false,
      "variation": false,
      "set": "rix",
      "set_name": "Rivals of Ixalan",
      "set_type": "expansion",
      "set_uri": "https://api.scryfall.com/sets/2f7e40fc-772d-4a85-bfdd-01653c41d0fa",
      "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Arix&unique=prints",
      "scryfall_set_uri": "https://scryfall.com/sets/rix?utm_source=api",
      "rulings_uri": "https://api.scryfall.com/cards/0104b5b3-9376-4ad7-9a77-3e564e9c42e6/rulings",
      "prints_search_uri": "https://api.scryfall.com/cards/search?order=released&q=oracleid%3Ab0b6be0c-41cf-4757-9f0e-87227b6ba6b3&unique=prints",
      "collector_number": "130",
      "digital": false,
      "rarity": "rare",
      "flavor_text": "The earth walks, strongest of all.",
      "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
      "artist": "Chase Stone",
      "artist_ids": [
        "2d753f61-5f5b-468e-97ea-8e0fdd347340"
      ],
      "illustration_id": "c3204342-9eea-41f8-9fe3-e6b36c5e3a9c",
      "border_color": "black",
      "frame": "2015",
      "full_art": false,
      "textless": false,
      "booster": true,
      "story_spotlight": false,
      "edhrec_rank": 1042,
      "prices": {
        "usd": "3.70",
        "usd_foil": "5.16",
        "eur": "2.55",
        "tix": "0.02"
      },
      "related_uris": {
        "gatherer": "https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=439787",
        "tcgplayer_decks": "https://decks.tcgplayer.com/magic/deck/search?contains=Ghalta%2C+Primal+Hunger&page=1&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
        "edhrec": "https://edhrec.com/route/?cc=Ghalta%2C+Primal+Hunger",
        "mtgtop8": "https://mtgtop8.com/search?MD_check=1&SB_check=1&cards=Ghalta%2C+Primal+Hunger"
      },
      "purchase_uris": {
        "tcgplayer": "https://shop.tcgplayer.com/product/productsearch?id=155579&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
        "cardmarket": "https://www.cardmarket.com/en/Magic/Products/Singles/Rivals-of-Ixalan/Ghalta-Primal-Hunger?referrer=scryfall&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall",
        "cardhoarder": "https://www.cardhoarder.com/cards/66713?affiliate_id=scryfall&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall"
      }
    }
  ]
}"#;
    let card_list: List<Card> = serde_json::from_str(json).expect("Unable to parse card list");

    // Resulting list should have only 1 entry
    assert_eq!(card_list.data.len(), 1);

    // The card in the list should be Ghalta
    assert_eq!(&card_list.data[0].gameplay.name, "Ghalta, Primal Hunger");
}
