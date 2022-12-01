use crate::structs::cards::{
    flood::FloodCard,
    island::{IslandCard, IslandCardName},
    treasure::{TreasureCard, TreasureCardType},
    Deck,
};

pub fn get_island_deck() -> Deck<IslandCard> {
    Deck::new(
        &IslandCardName::iter()
            .map(IslandCard::from_name)
            .collect::<Vec<_>>(),
    )
}

pub fn get_flood_deck() -> Deck<FloodCard> {
    Deck::new(
        &IslandCardName::iter()
            .map(FloodCard::from_name)
            .collect::<Vec<_>>(),
    )
}

pub fn get_treasure_deck() -> Deck<TreasureCard> {
    Deck::new(&[
        TreasureCard::new(TreasureCardType::Earth),
        TreasureCard::new(TreasureCardType::Earth),
        TreasureCard::new(TreasureCardType::Earth),
        TreasureCard::new(TreasureCardType::Earth),
        TreasureCard::new(TreasureCardType::Earth),
        TreasureCard::new(TreasureCardType::Wind),
        TreasureCard::new(TreasureCardType::Wind),
        TreasureCard::new(TreasureCardType::Wind),
        TreasureCard::new(TreasureCardType::Wind),
        TreasureCard::new(TreasureCardType::Wind),
        TreasureCard::new(TreasureCardType::Fire),
        TreasureCard::new(TreasureCardType::Fire),
        TreasureCard::new(TreasureCardType::Fire),
        TreasureCard::new(TreasureCardType::Fire),
        TreasureCard::new(TreasureCardType::Fire),
        TreasureCard::new(TreasureCardType::Ocean),
        TreasureCard::new(TreasureCardType::Ocean),
        TreasureCard::new(TreasureCardType::Ocean),
        TreasureCard::new(TreasureCardType::Ocean),
        TreasureCard::new(TreasureCardType::Ocean),
        TreasureCard::new(TreasureCardType::WaterRise),
        TreasureCard::new(TreasureCardType::WaterRise),
        TreasureCard::new(TreasureCardType::WaterRise),
        TreasureCard::new(TreasureCardType::HelicopterLift),
        TreasureCard::new(TreasureCardType::HelicopterLift),
        TreasureCard::new(TreasureCardType::HelicopterLift),
        TreasureCard::new(TreasureCardType::Sandbag),
        TreasureCard::new(TreasureCardType::Sandbag),
    ])
}
