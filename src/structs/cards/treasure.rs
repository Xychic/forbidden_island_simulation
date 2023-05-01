use std::slice::Iter;

use super::{Card, CardType, Deck};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreasureType {
    Earth,
    Wind,
    Fire,
    Ocean,
}

impl TreasureType {
    const fn _all() -> [TreasureType; 4] {
        [
            TreasureType::Earth,
            TreasureType::Wind,
            TreasureType::Fire,
            TreasureType::Ocean,
        ]
    }

    pub fn _iter() -> Iter<'static, TreasureType> {
        static ALL_TREASURE_TYPES: [TreasureType; 4] = TreasureType::_all();
        ALL_TREASURE_TYPES.iter()
    }

    pub fn get_name(self) -> String {
        match self {
            TreasureType::Earth => "Earth Stone",
            TreasureType::Wind => "Statue of Wind",
            TreasureType::Fire => "Crystal of Fire",
            TreasureType::Ocean => "Ocean's Chalice",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialActionType {
    Sandbag,
    HelicopterLift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreasureCardType {
    Treasure(TreasureType),
    SpecialAction(SpecialActionType),
    WaterRise,
}

impl TreasureCardType {
    pub const fn all() -> [TreasureCardType; 28] {
        [
            TreasureCardType::Treasure(TreasureType::Earth),
            TreasureCardType::Treasure(TreasureType::Earth),
            TreasureCardType::Treasure(TreasureType::Earth),
            TreasureCardType::Treasure(TreasureType::Earth),
            TreasureCardType::Treasure(TreasureType::Earth),
            TreasureCardType::Treasure(TreasureType::Wind),
            TreasureCardType::Treasure(TreasureType::Wind),
            TreasureCardType::Treasure(TreasureType::Wind),
            TreasureCardType::Treasure(TreasureType::Wind),
            TreasureCardType::Treasure(TreasureType::Wind),
            TreasureCardType::Treasure(TreasureType::Fire),
            TreasureCardType::Treasure(TreasureType::Fire),
            TreasureCardType::Treasure(TreasureType::Fire),
            TreasureCardType::Treasure(TreasureType::Fire),
            TreasureCardType::Treasure(TreasureType::Fire),
            TreasureCardType::Treasure(TreasureType::Ocean),
            TreasureCardType::Treasure(TreasureType::Ocean),
            TreasureCardType::Treasure(TreasureType::Ocean),
            TreasureCardType::Treasure(TreasureType::Ocean),
            TreasureCardType::Treasure(TreasureType::Ocean),
            TreasureCardType::WaterRise,
            TreasureCardType::WaterRise,
            TreasureCardType::WaterRise,
            TreasureCardType::SpecialAction(SpecialActionType::HelicopterLift),
            TreasureCardType::SpecialAction(SpecialActionType::HelicopterLift),
            TreasureCardType::SpecialAction(SpecialActionType::HelicopterLift),
            TreasureCardType::SpecialAction(SpecialActionType::Sandbag),
            TreasureCardType::SpecialAction(SpecialActionType::Sandbag),
        ]
    }

    pub fn iter() -> Iter<'static, TreasureCardType> {
        static ALL_CARD_TYPES: [TreasureCardType; 28] = TreasureCardType::all();
        ALL_CARD_TYPES.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct TreasureCard {
    treasure_type: TreasureCardType,
}

#[allow(dead_code)]
impl TreasureCard {
    pub fn new(&treasure_type: &TreasureCardType) -> TreasureCard {
        TreasureCard { treasure_type }
    }

    pub fn get_type(&self) -> &TreasureCardType {
        &self.treasure_type
    }
}

impl Card for TreasureCard {
    fn card_type() -> super::CardType {
        CardType::Treasure
    }

    fn get_deck() -> super::Deck<Self> {
        Deck::from(
            &TreasureCardType::iter()
                .map(TreasureCard::new)
                .collect::<Vec<_>>(),
        )
    }
}
