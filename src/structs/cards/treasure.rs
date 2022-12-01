use super::{Card, CardType};

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
}
