use super::{island::IslandCardName, Card, CardType, Deck};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn get_type(&self) -> IslandCardName {
        self.island_card
    }
}

impl Card for FloodCard {
    fn card_type() -> CardType {
        CardType::Flood
    }

    fn get_deck() -> super::Deck<Self> {
        Deck::from(
            &IslandCardName::iter()
                .map(FloodCard::from_name)
                .collect::<Vec<_>>(),
        )
    }

    fn as_string(&self) -> String {
        self.get_type().name()
    }
}
