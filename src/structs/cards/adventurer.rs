use std::slice::Iter;

use super::{island::IslandCardName, treasure::TreasureCard, Card, CardType, Deck};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdventurerCard {
    card_type: AdventurerCardType,
    treasure_hand: Deck<TreasureCard>,
}

#[allow(dead_code)]
impl AdventurerCard {
    pub fn new(&card_type: &AdventurerCardType) -> Self {
        AdventurerCard {
            card_type,
            treasure_hand: Deck::with_capacity(10),
        }
    }

    pub fn get_start_card(&self) -> IslandCardName {
        match self.card_type {
            AdventurerCardType::Explorer => IslandCardName::CopperGate,
            AdventurerCardType::Pilot => IslandCardName::FoolsLanding,
            AdventurerCardType::Engineer => IslandCardName::BronzeGate,
            AdventurerCardType::Diver => IslandCardName::IronGate,
            AdventurerCardType::Messenger => IslandCardName::SilverGate,
            AdventurerCardType::Navigator => IslandCardName::GoldGate,
        }
    }

    pub fn receive_card(&mut self, card: TreasureCard) {
        self.treasure_hand.insert(card);
    }

    pub fn get_card_count(&self) -> usize {
        self.treasure_hand.len()
    }

    pub fn get_type(&self) -> AdventurerCardType {
        self.card_type
    }

    pub fn get_hand(&self) -> &Deck<TreasureCard> {
        &self.treasure_hand
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdventurerCardType {
    Explorer,
    Pilot,
    Engineer,
    Diver,
    Messenger,
    Navigator,
}

impl AdventurerCardType {
    pub const fn all() -> [AdventurerCardType; 6] {
        [
            AdventurerCardType::Explorer,
            AdventurerCardType::Pilot,
            AdventurerCardType::Engineer,
            AdventurerCardType::Diver,
            AdventurerCardType::Messenger,
            AdventurerCardType::Navigator,
        ]
    }

    pub fn iter() -> Iter<'static, AdventurerCardType> {
        static ALL_CARD_NAMES: [AdventurerCardType; 6] = AdventurerCardType::all();
        ALL_CARD_NAMES.iter()
    }
}

impl Card for AdventurerCard {
    fn card_type() -> CardType {
        CardType::Adventurer
    }

    fn get_deck() -> Deck<Self> {
        Deck::from(
            &AdventurerCardType::iter()
                .map(AdventurerCard::new)
                .collect::<Vec<_>>(),
        )
    }
}
