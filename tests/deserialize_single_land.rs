use dcc_scryfall::Card;

#[test]
pub fn deserialize_single_land() {
    let json = r#"{  "object": "card",
  "id": "bd7567df-b4d8-41a8-8eac-c05afa784bfe",
  "oracle_id": "b76d1ae6-ad1d-4bac-b4c3-2e03e0e84d9b",
  "multiverse_ids": [
    382860
  ],
  "mtgo_id": 53079,
  "mtgo_foil_id": 53080,
  "name": "Bayou",
  "lang": "en",
  "released_at": "2014-06-16",
  "uri": "https://api.scryfall.com/cards/bd7567df-b4d8-41a8-8eac-c05afa784bfe",
  "scryfall_uri": "https://scryfall.com/card/vma/293/bayou?utm_source=api",
  "layout": "normal",
  "highres_image": true,
  "image_uris": {
    "small": "https://img.scryfall.com/cards/small/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.jpg?1562933075",
    "normal": "https://img.scryfall.com/cards/normal/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.jpg?1562933075",
    "large": "https://img.scryfall.com/cards/large/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.jpg?1562933075",
    "png": "https://img.scryfall.com/cards/png/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.png?1562933075",
    "art_crop": "https://img.scryfall.com/cards/art_crop/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.jpg?1562933075",
    "border_crop": "https://img.scryfall.com/cards/border_crop/front/b/d/bd7567df-b4d8-41a8-8eac-c05afa784bfe.jpg?1562933075"
  },
  "mana_cost": "",
  "cmc": 0.0,
  "type_line": "Land — Swamp Forest",
  "oracle_text": "({T}: Add {B} or {G}.)",
  "colors": [

  ],
  "color_identity": [
    "B",
    "G"
  ],
  "legalities": {
    "standard": "not_legal",
    "future": "not_legal",
    "historic": "not_legal",
    "modern": "not_legal",
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
    "mtgo"
  ],
  "reserved": true,
  "foil": true,
  "nonfoil": true,
  "oversized": false,
  "promo": false,
  "reprint": true,
  "variation": false,
  "set": "vma",
  "set_name": "Vintage Masters",
  "set_type": "masters",
  "set_uri": "https://api.scryfall.com/sets/a944551a-73fa-41cd-9159-e8d0e4674403",
  "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Avma&unique=prints",
  "scryfall_set_uri": "https://scryfall.com/sets/vma?utm_source=api",
  "rulings_uri": "https://api.scryfall.com/cards/bd7567df-b4d8-41a8-8eac-c05afa784bfe/rulings",
  "prints_search_uri": "https://api.scryfall.com/cards/search?order=released&q=oracleid%3Ab76d1ae6-ad1d-4bac-b4c3-2e03e0e84d9b&unique=prints",
  "collector_number": "293",
  "digital": true,
  "rarity": "rare",
  "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
  "artist": "Karl Kopinski",
  "artist_ids": [
    "0fad2c48-a56e-4a0b-b512-224f6f238f20"
  ],
  "illustration_id": "0112e448-4c26-4adb-a15e-743dad3ff5ba",
  "border_color": "black",
  "frame": "2015",
  "full_art": false,
  "textless": false,
  "booster": true,
  "story_spotlight": false,
  "edhrec_rank": 488,
  "prices": {
    "usd": null,
    "usd_foil": null,
    "eur": null,
    "tix": "3.13"
  },
  "related_uris": {
    "gatherer": "https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=382860",
    "tcgplayer_decks": "https://decks.tcgplayer.com/magic/deck/search?contains=Bayou&page=1&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
    "edhrec": "https://edhrec.com/route/?cc=Bayou",
    "mtgtop8": "https://mtgtop8.com/search?MD_check=1&SB_check=1&cards=Bayou"
  },
  "purchase_uris": {
    "tcgplayer": "https://shop.tcgplayer.com/productcatalog/product/show?ProductName=Bayou&partner=Scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
    "cardmarket": "https://www.cardmarket.com/en/Magic/Products/Search?referrer=scryfall&searchString=Bayou&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall",
    "cardhoarder": "https://www.cardhoarder.com/cards/53079?affiliate_id=scryfall&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall"
  }
}"#;

    let card: Card = serde_json::from_str(json).expect("Unable to parse land");

    assert_eq!(card.gameplay.name, "Bayou");
}
