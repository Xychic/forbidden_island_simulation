pub mod moves;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rand::Rng;

use crate::structs::board::ISLAND_COORDS;

use self::moves::{Action, ActionType};

use super::{
    board::Board,
    cards::{
        adventurer::{AdventurerCard, AdventurerCardType},
        flood::FloodCard,
        island::{IslandCard, IslandCardState},
        treasure::{TreasureCard, TreasureCardType, TreasureType},
        Card, Deck,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adventurer {
    card: AdventurerCardType,
    pub pos: (usize, usize),
    hand: Deck<TreasureCard>,
    used_pilot_move: bool,
}

impl Adventurer {
    pub fn new(card: AdventurerCard, board: &Board) -> Adventurer {
        let start_card = card.get_start_card();
        Adventurer {
            card: card.get_type(),
            pos: board.get_location(&start_card),
            hand: Deck::with_capacity(10),
            used_pilot_move: false,
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

    pub fn remove_card(&mut self, index: usize) -> TreasureCard {
        self.hand.pop_card(index).unwrap()
    }

    pub fn find_card(&self, card_type: &TreasureCardType) -> Option<usize> {
        for (i, card) in self.hand.iter().enumerate() {
            if card.get_type() == card_type {
                return Some(i);
            }
        }
        None
    }

    pub fn receive_card(&mut self, card: TreasureCard) {
        self.hand.insert(card);
    }

    pub fn move_to(&mut self, &(x, y): &(usize, usize)) {
        if self.card == AdventurerCardType::Pilot
            && self.pos.0.abs_diff(x) + self.pos.1.abs_diff(y) > 1
        {
            self.used_pilot_move = true;
        }
        self.pos = (x, y);
    }
}

#[derive(Debug)]
pub struct Game<R: Rng> {
    pub rng: R,
    pub treasure_deck: Deck<TreasureCard>,
    pub treasure_discard_deck: Deck<TreasureCard>,
    pub flood_deck: Deck<FloodCard>,
    pub flood_discard_deck: Deck<FloodCard>,
    pub adventurers: HashMap<AdventurerCardType, Adventurer>, // TODO Order of hashmap not consistent
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

    pub fn get_moves(&self, adventurer: &Adventurer) -> Vec<(usize, usize)> {
        let adventurer_type = adventurer.card;
        let (x, y) = adventurer.pos;
        match (adventurer_type, adventurer.used_pilot_move) {
            (AdventurerCardType::Explorer, _) => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            (AdventurerCardType::Pilot, false) => Vec::from(ISLAND_COORDS),
            _ => vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .filter(|&pos @ &(px, py)| {
            pos != &(x, y)
                && ISLAND_COORDS.contains(pos)
                && (self.board.get_card(&(px, py)).unwrap().state() != &IslandCardState::Sunk
                    || adventurer_type == AdventurerCardType::Diver)
        })
        .copied()
        .collect()
    }

    pub fn get_shorable(
        &self,
        adventurer_type: &AdventurerCardType,
        &(x, y): &(usize, usize),
    ) -> Vec<(usize, usize)> {
        match adventurer_type {
            AdventurerCardType::Explorer => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            _ => vec![(x, y + 1), (x - 1, y), (x, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .filter(|&pos| {
            ISLAND_COORDS.contains(pos)
                && self.board.get_card(pos).unwrap().state() == &IslandCardState::Flooded
        })
        .copied()
        .collect()
    }

    pub fn do_action<F: Fn(&Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
    ) {
        let adventurer = self.adventurers.get(adventurer_type).unwrap();
        let mut actions = Vec::new();
        // Move
        for pos in self.get_moves(adventurer) {
            actions.push(Action::new(
                ActionType::Move(pos),
                format!("Move to {:?}", self.board.get_card(&pos).unwrap().name()),
            ));
        }

        // Navigator Moves
        if adventurer.card == AdventurerCardType::Navigator {
            for t in self
                .adventurers
                .keys()
                .into_iter()
                .filter(|&a| a != &AdventurerCardType::Navigator)
            {
                for pos in self.get_moves(self.adventurers.get(t).unwrap()) {
                    actions.push(Action::new(
                        ActionType::NavigatorMove(t.to_owned(), pos),
                        format!(
                            "Move {:?} to {:?}",
                            t,
                            self.board.get_card(&pos).unwrap().name()
                        ),
                    ));
                }
            }
        }

        // Shore up
        // TODO Engineer can shore up 2
        for pos in self.get_shorable(&adventurer.card, &adventurer.pos) {
            actions.push(Action::new(
                ActionType::ShoreUp(pos),
                format!("Shore up {:?}", self.board.get_card(&pos).unwrap().name()),
            ))
        }

        // Give card
        for a in self
            .adventurers
            .values()
            .filter(|a| a.card != adventurer.card && a.pos == adventurer.pos)
        {
            for (i, c) in adventurer
                .get_hand()
                .iter()
                .enumerate()
                .unique_by(|&(_, x)| x)
            {
                actions.push(Action::new(
                    ActionType::GiveCard(i, *a.get_type()),
                    format!("Give {:?} card to {:?}", c.get_type(), a.get_type()),
                ));
            }
        }

        // Capture a treasure

        // Play special action card - doesn't use turn

        // End Turn
        // TODO always available

        // Choose option
        let action_strings: Vec<_> = actions
            .iter()
            .enumerate()
            .map(|(i, a)| (format!("{i}: {}", a.description())))
            .collect();
        let choice = chooser(&action_strings);
        match actions[choice].action_type() {
            ActionType::Move(pos) => self
                .adventurers
                .get_mut(adventurer_type)
                .unwrap()
                .move_to(pos),
            ActionType::NavigatorMove(t, pos) => {
                let adventurer = self.adventurers.get_mut(t).unwrap();
                adventurer.move_to(pos);
            }
            ActionType::ShoreUp(pos) => self.board.shore_up(pos),
            ActionType::GiveCard(i, a) => {
                let card = self
                    .adventurers
                    .get_mut(adventurer_type)
                    .unwrap()
                    .remove_card(*i);
                self.adventurers.get_mut(a).unwrap().receive_card(card);
            }
            ActionType::CaptureTreasure => todo!(),
            ActionType::PlayActionCard => todo!(),
            ActionType::EndTurn => todo!(),
        }
    }
}
