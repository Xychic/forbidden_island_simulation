use super::{Card, CardType, Deck};

#[derive(Debug, Clone, Copy)]
pub enum TreasureType {
    Earth,
    Wind,
    Fire,
    Ocean,
}

#[derive(Debug, Clone)]
pub enum TreasureCardType {
    Earth,
    Wind,
    Fire,
    Ocean,
    Sandbag,
    HelicopterLift,
    WaterRise,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TreasureCard {
    treasure_type: TreasureCardType,
}

#[allow(dead_code)]
impl TreasureCard {
    pub fn new(treasure_type: TreasureCardType) -> TreasureCard {
        TreasureCard { treasure_type }
    }
}

impl Card for TreasureCard {
    fn card_type() -> super::CardType {
        CardType::Treasure
    }

    fn get_deck() -> super::Deck<Self> {
        Deck {
            cards: vec![
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
            ],
        }
    }
}
