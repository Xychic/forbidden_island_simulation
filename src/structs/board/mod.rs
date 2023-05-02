use std::collections::HashMap;

use super::cards::{
    island::{IslandCard, IslandCardName, IslandCardState},
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
pub struct Board {
    board: [[Option<IslandCard>; 6]; 6],
    locations: HashMap<IslandCardName, (usize, usize)>,
}

impl Board {
    pub fn new(island_deck: &mut Deck<IslandCard>) -> Board {
        assert_eq!(island_deck.len(), ISLAND_COORDS.len());
        let mut board = [[None; 6]; 6];
        let mut locations = HashMap::with_capacity(36);

        for &(x, y) in ISLAND_COORDS.iter() {
            board[y][x] = island_deck.pop_next();
            locations.insert(board[y][x].unwrap().name(), (x, y));
        }
        Board { board, locations }
    }

    pub fn show(&self) -> String {
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
        res.iter()
            .map(|row| row.join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn sink(&mut self, card: &IslandCardName) -> bool {
        let (x, y) = *self.locations.get(card).unwrap();
        if let Some(card) = &mut self.board[y][x] {
            (*card).sink();
            return card.state() == &IslandCardState::Sunk;
        }
        panic!()
    }

    pub fn shore_up(&mut self, &(x, y): &(usize, usize)) {
        if let Some(card) = &mut self.board[y][x] {
            (*card).raise();
        }
    }

    pub fn get_location(&self, card: &IslandCardName) -> (usize, usize) {
        *self.locations.get(card).unwrap()
    }

    pub fn get_by_type(&self, card: &IslandCardName) -> IslandCard {
        self.get_card(&self.get_location(card)).unwrap()
    }

    pub fn get_card(&self, coord @ &(x, y): &(usize, usize)) -> Option<IslandCard> {
        if ISLAND_COORDS.contains(coord) {
            self.board[y][x]
        } else {
            None
        }
    }

    pub fn has_shorable(&self) -> bool {
        for row in self.board {
            for card in row.into_iter().flatten() {
                if card.state() == &IslandCardState::Flooded {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_shorable(&self) -> Vec<IslandCardName> {
        let mut shorable = Vec::new();
        for row in self.board {
            for card in row.into_iter().flatten() {
                if card.state() == &IslandCardState::Flooded {
                    shorable.push(card.name());
                }
            }
        }
        shorable
    }
}
