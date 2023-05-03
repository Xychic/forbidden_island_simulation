use std::slice::Iter;

use crate::structs::board::Board;

use super::{island::IslandCardName, treasure::TreasureCard, Card, Deck};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdventurerCard {
    adventurer_type: AdventurerCardType,
}

#[allow(dead_code)]
impl AdventurerCard {
    pub fn new(&adventurer_type: &AdventurerCardType) -> Self {
        AdventurerCard { adventurer_type }
    }

    pub fn get_start_card(&self) -> IslandCardName {
        match self.adventurer_type {
            AdventurerCardType::Explorer => IslandCardName::CopperGate,
            AdventurerCardType::Pilot => IslandCardName::FoolsLanding,
            AdventurerCardType::Engineer => IslandCardName::BronzeGate,
            AdventurerCardType::Diver => IslandCardName::IronGate,
            AdventurerCardType::Messenger => IslandCardName::SilverGate,
            AdventurerCardType::Navigator => IslandCardName::GoldGate,
        }
    }

    pub fn get_type(&self) -> AdventurerCardType {
        self.adventurer_type
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
    fn get_deck() -> Deck<Self> {
        Deck::from(
            &AdventurerCardType::iter()
                .map(AdventurerCard::new)
                .collect::<Vec<_>>(),
        )
    }

    fn as_string(&self) -> String {
        format!("{:?}", self.adventurer_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adventurer {
    card: AdventurerCardType,
    pos: (usize, usize),
    hand: Deck<TreasureCard>,
}

impl Adventurer {
    pub fn new(card: AdventurerCard, board: &Board) -> Adventurer {
        let start_card = card.get_start_card();
        Adventurer {
            card: card.get_type(),
            pos: board.get_location(&start_card),
            hand: Deck::with_capacity(10),
        }
    }

    pub fn get_type(&self) -> &AdventurerCardType {
        &self.card
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.len()
    }

    pub fn get_hand(&self) -> &Deck<TreasureCard> {
        &self.hand
    }

    pub fn has_cards(&self) -> bool {
        !self.hand.is_empty()
    }

    pub fn remove_card(&mut self, index: usize) -> TreasureCard {
        self.hand.pop_card(index).unwrap()
    }

    pub fn receive_card(&mut self, card: TreasureCard) {
        self.hand.insert(card);
    }

    pub fn get_pos(&self) -> &(usize, usize) {
        &self.pos
    }

    pub fn move_to(&mut self, dest: (usize, usize)) {
        self.pos = dest
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::cards::island::IslandCard;

    use super::*;

    #[test]
    fn test_fn_new() {
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Explorer
            },
            AdventurerCard::new(&AdventurerCardType::Explorer)
        );
    }

    #[test]
    fn test_fn_get_start_card() {
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Explorer
            }
            .get_start_card(),
            IslandCardName::CopperGate
        );
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Pilot
            }
            .get_start_card(),
            IslandCardName::FoolsLanding
        );
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Engineer
            }
            .get_start_card(),
            IslandCardName::BronzeGate
        );
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Diver
            }
            .get_start_card(),
            IslandCardName::IronGate
        );
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Messenger
            }
            .get_start_card(),
            IslandCardName::SilverGate
        );
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Navigator
            }
            .get_start_card(),
            IslandCardName::GoldGate
        );
    }

    #[test]
    fn test_fn_get_type() {
        assert_eq!(
            AdventurerCard {
                adventurer_type: AdventurerCardType::Explorer
            }
            .get_type(),
            AdventurerCardType::Explorer
        );
    }

    #[test]
    fn test_fn_adventurer_new() {
        let mut island_deck = IslandCard::get_deck();
        let board = Board::new(&mut island_deck);
        assert_eq!(
            Adventurer {
                card: AdventurerCardType::Explorer,
                pos: board.get_location(&IslandCardName::CopperGate),
                hand: Deck::with_capacity(10),
            },
            Adventurer::new(AdventurerCard::new(&AdventurerCardType::Explorer), &board)
        );
    }
}
