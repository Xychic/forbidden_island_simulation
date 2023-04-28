use std::collections::{HashMap, HashSet};

use rand::Rng;

use super::{
    board::Board,
    cards::{
        adventurer::{AdventurerCard, AdventurerCardType},
        flood::FloodCard,
        island::IslandCard,
        treasure::{TreasureCard, TreasureCardType, TreasureType},
        Card, Deck,
    },
};

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

    pub fn get_card_count(&self) -> usize {
        self.hand.len()
    }

    pub fn get_hand(&self) -> &Deck<TreasureCard> {
        &self.hand
    }

    pub fn receive_card(&mut self, card: TreasureCard) {
        self.hand.insert(card);
    }
}

#[derive(Debug)]
pub struct Game<R: Rng> {
    pub rng: R,
    pub treasure_deck: Deck<TreasureCard>,
    pub treasure_discard_deck: Deck<TreasureCard>,
    pub flood_deck: Deck<FloodCard>,
    pub flood_discard_deck: Deck<FloodCard>,
    pub adventurers: HashMap<AdventurerCardType, Adventurer>,
    pub board: Board,
    pub water_level: usize,
    captured_treasures: HashSet<TreasureType>,
}

impl<R: Rng> Game<R> {
    pub fn new(mut rng: R, adventurers_count: usize, water_level: usize) -> Game<R> {
        let mut island_deck = IslandCard::get_deck();
        let mut adventurer_deck = AdventurerCard::get_deck();

        let mut treasure_deck = TreasureCard::get_deck();
        let mut treasure_discard_deck = Deck::with_capacity(28);
        let mut flood_deck = FloodCard::get_deck();
        let mut flood_discard_deck = Deck::with_capacity(24);

        // Shuffle the decks
        island_deck.shuffle(&mut rng);
        treasure_deck.shuffle(&mut rng);
        flood_deck.shuffle(&mut rng);
        adventurer_deck.shuffle(&mut rng);

        // Setup 1 - Create the island
        let mut board = Board::new(&mut island_deck);

        // Setup 4 - The Island Starts to sink
        for _ in 0..6 {
            let to_sink = flood_deck.pop_next().unwrap();
            board.sink(&to_sink.get_type());
            flood_discard_deck.insert(to_sink);
        }

        // Setup 5 - The Adventurers Appear
        let mut adventurers: HashMap<_, _> = (0..adventurers_count)
            .map(|_| {
                let adventurer = adventurer_deck.pop_next().unwrap();
                (adventurer.get_type(), Adventurer::new(adventurer, &board))
            })
            .collect();

        // Setup 6 - Hand out Treasure Deck Cards
        for _ in 0..2 {
            for (_, adventurer) in adventurers.iter_mut() {
                // adventurer.receive_card()
                loop {
                    let card = treasure_deck.pop_next().unwrap();
                    if card.get_type() == &TreasureCardType::WaterRise {
                        treasure_discard_deck.insert(card);
                    } else {
                        adventurer.receive_card(card);
                        break;
                    }
                }
            }
        }

        treasure_deck.stack(&mut treasure_discard_deck);
        treasure_deck.shuffle(&mut rng);

        Game {
            rng,
            treasure_deck,
            treasure_discard_deck,
            flood_deck,
            flood_discard_deck,
            adventurers,
            board,
            water_level,
            captured_treasures: HashSet::with_capacity(4),
        }
    }
}
