use super::{island::IslandCardName, Card, CardType};

#[derive(Debug, Clone)]
pub struct FloodCard {
    island_card: IslandCardName,
}

#[allow(dead_code)]
impl FloodCard {
    pub fn new(island_card: IslandCardName) -> FloodCard {
        FloodCard { island_card }
    }

    pub fn from_name(name: &IslandCardName) -> FloodCard {
        FloodCard {
            island_card: name.to_owned(),
        }
    }
}

impl Card for FloodCard {
    fn card_type() -> CardType {
        CardType::Flood
    }
}
