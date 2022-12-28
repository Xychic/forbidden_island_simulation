#[macro_use]
pub mod cards;

#[allow(dead_code)]
pub mod game_board {

    use std::collections::{HashMap, HashSet};

    use rand::Rng;

    use crate::structs::cards::Card;

    use super::cards::{
        adventurer::{self, AdventurerCard, AdventurerCardType},
        flood::FloodCard,
        island::{IslandCard, IslandCardName, IslandCardState},
        treasure::{self, TreasureCard, TreasureCardType, TreasureType},
        Deck,
    };

    pub const ISLAND_COORDS: [(usize, usize); 24] = [
        (2, 0),
        (3, 0),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (0, 2),
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
        (4, 3),
        (5, 3),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
        (2, 5),
        (3, 5),
    ];

    #[derive(Debug)]
    pub struct GameBoard {
        board: [[Option<IslandCard>; 6]; 6],
        water_level: usize,
        island_card_locations: HashMap<IslandCardName, (usize, usize)>,
        pub adventurer_locations: HashMap<AdventurerCardType, (AdventurerCard, (usize, usize))>,
        treasure_deck: Deck<TreasureCard>,
        treasure_discard_deck: Deck<TreasureCard>,
        flood_deck: Deck<FloodCard>,
        flood_discard_deck: Deck<FloodCard>,
        captured_treasures: HashSet<TreasureType>,
    }

    impl GameBoard {
        pub fn new<R>(rng: &mut R, water_level: usize) -> GameBoard
        where
            R: Rng,
        {
            assert!(water_level <= 4, "Water level cannot start more than 4");

            let mut island_deck = IslandCard::get_deck();
            island_deck.shuffle(rng);

            let mut adventurer_deck = AdventurerCard::get_deck();
            adventurer_deck.shuffle(rng);

            let mut treasure_deck = TreasureCard::get_deck();
            treasure_deck.shuffle(rng);

            let mut flood_deck = FloodCard::get_deck();
            flood_deck.shuffle(rng);

            assert_eq!(island_deck.len(), ISLAND_COORDS.len());
            let mut board = [[None; 6]; 6];
            let mut island_card_locations = HashMap::with_capacity(36);
            let mut adventurer_locations = HashMap::with_capacity(4);

            let mut adventurers: Vec<_> = (0..4)
                .map(|_| adventurer_deck.pop_next().unwrap())
                .collect();

            for &(x, y) in ISLAND_COORDS.iter() {
                let card = island_deck.pop_next();
                assert!(card.is_some(), "Missing island cards!");
                board[y][x] = card;

                let card_name = card.unwrap().name();
                island_card_locations.insert(card_name, (x, y));

                if let Some(index) =
                    (0..adventurers.len()).find(|&i| adventurers[i].get_start_card() == card_name)
                {
                    let adventurer = adventurers.remove(index);
                    adventurer_locations.insert(adventurer.get_type(), (adventurer, (x, y)));
                }
            }
            GameBoard {
                board,
                water_level,
                island_card_locations,
                adventurer_locations,
                treasure_discard_deck: Deck::with_capacity(treasure_deck.len()),
                treasure_deck,
                flood_discard_deck: Deck::with_capacity(flood_deck.len()),
                flood_deck,
                captured_treasures: HashSet::with_capacity(4),
            }
        }

        pub fn show_board(&self) -> String {
            let mut res = vec![Vec::with_capacity(self.board[0].len()); self.board.len() * 3];

            let tile_strs: Vec<Vec<_>> = self
                .board
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Some(card) => card.tile_str(),
                            None => "     \n     \n     ".to_owned(),
                        })
                        .collect()
                })
                .collect();

            for (row_index, row) in tile_strs.iter().enumerate() {
                for tile in row {
                    for (part_index, part) in tile.split('\n').enumerate() {
                        res[row_index * 3 + part_index].push(part);
                    }
                }
            }

            let board_string = res
                .iter()
                .map(|row| row.join(" "))
                .fold(String::new(), |acc, val| acc + &val + "\n");

            self.adventurer_locations
                .iter()
                .map(|(adventurer, &(_, (x, y)))| {
                    format!("{:?}:  {:?}", adventurer, self.board[y][x].unwrap().name())
                })
                .fold(board_string, |acc, val| acc + "\n" + &val)
        }

        pub fn sink(&mut self, card: &IslandCardName) {
            let (x, y) = *self.island_card_locations.get(card).unwrap();
            if let Some(card) = &mut self.board[y][x] {
                (*card).sink();
            }
        }

        pub fn shore_up(&mut self, card: &IslandCardName) {
            let (x, y) = self.get_location(card);
            if let Some(card) = &mut self.board[y][x] {
                (*card).raise();
            }
        }

        pub fn get_location(&self, card: &IslandCardName) -> (usize, usize) {
            *self.island_card_locations.get(card).unwrap()
        }

        pub fn get_card(&self, coord @ &(x, y): &(usize, usize)) -> Option<IslandCard> {
            if ISLAND_COORDS.contains(coord) {
                self.board[y][x]
            } else {
                None
            }
        }

        pub fn get_adjacent(&self, card: &IslandCardName) -> Vec<IslandCardName> {
            let (x, y) = self.get_location(card);
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter_map(|coord| {
                    if let Some(card) = self.get_card(coord) {
                        if *card.state() != IslandCardState::Sunk {
                            return Some(card.name());
                        }
                    }
                    None
                })
                .collect()
        }

        pub fn get_options(&self, adventurer: &AdventurerCardType, moves_left: usize) {
            let (adventurer_struct, (x, y)) = self.adventurer_locations.get(adventurer).unwrap();
            let moves = self.get_moves(adventurer);

            if let Some(treasure_type) = TreasureType::iter().find(|&treasure_type| {
                adventurer_struct
                    .get_hand()
                    .iter()
                    .filter(|card| match card.get_type() {
                        TreasureCardType::Treasure(t) => &t == treasure_type,
                        _ => false,
                    })
                    .count()
                    >= 4
                    && self.board[*y][*x].unwrap().can_retrieve(treasure_type)
            }) {
                dbg!(treasure_type);
            }
        }

        pub fn get_moves(&self, adventurer: &AdventurerCardType) -> Vec<(usize, usize)> {
            let &(_, (x, y)) = self.adventurer_locations.get(adventurer).unwrap();

            match adventurer {
                AdventurerCardType::Explorer => vec![
                    (x + 1, y + 1),
                    (x, y + 1),
                    (x - 1, y + 1),
                    (x + 1, y),
                    (x - 1, y),
                    (x + 1, y - 1),
                    (x, y - 1),
                    (x - 1, y - 1),
                ],
                AdventurerCardType::Pilot => Vec::from(ISLAND_COORDS),
                _ => vec![(x, y + 1), (x + 1, y), (x - 1, y), (x, y - 1)],
            }
            .iter()
            .filter(|&pos @ &(x, y)| {
                ISLAND_COORDS.contains(pos)
                    && (self.board[y][x].unwrap().state() == &IslandCardState::Normal
                        || adventurer == &AdventurerCardType::Diver)
            })
            .copied().collect()
        }

        /// Returns `true` if draws a water rise card
        pub fn draw_treasure_card<R>(&mut self, adventurer: AdventurerCardType, rng: &mut R) -> bool
        where
            R: Rng,
        {
            let card = self.treasure_deck.pop_next().unwrap();

            if self.treasure_deck.is_empty() {
                self.treasure_discard_deck.shuffle(rng);
                for _ in 0..self.treasure_discard_deck.len() {
                    self.treasure_deck
                        .insert(self.treasure_discard_deck.pop_next().unwrap());
                }
            }

            if card.get_type() != TreasureCardType::WaterRise {
                self.adventurer_locations
                    .entry(adventurer)
                    .and_modify(|(adventurer, _)| adventurer.receive_card(card));
                false
            } else {
                true
            }
        }
    }
}
