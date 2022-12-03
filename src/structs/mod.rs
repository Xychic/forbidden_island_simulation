pub mod cards;

pub mod game_board {

    use std::collections::HashMap;

    use super::cards::{
        island::{IslandCard, IslandCardName},
        Deck,
    };

    const ISLAND_COORDS: [(usize, usize); 24] = [
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
        locations: HashMap<IslandCardName, (usize, usize)>,
    }

    impl GameBoard {
        pub fn new(island_deck: &mut Deck<IslandCard>) -> GameBoard {
            assert_eq!(island_deck.len(), ISLAND_COORDS.len());
            let mut board = [[None; 6]; 6];
            let mut locations = HashMap::with_capacity(36);

            for &(x, y) in ISLAND_COORDS.iter() {
                board[y][x] = island_deck.pop_next();
                locations.insert(*board[y][x].unwrap().name(), (x, y));
            }
            GameBoard { board, locations }
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
            res.iter()
                .map(|row| row.join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        }

        pub fn sink(&mut self, card: &IslandCardName) {
            let (x, y) = *self.locations.get(card).unwrap();
            if let Some(card) = &mut self.board[y][x] {
                (*card).sink();
            }
        }

        pub fn shore_up(&mut self, card: &IslandCardName) {
            let (x, y) = *self.locations.get(card).unwrap();
            if let Some(card) = &mut self.board[y][x] {
                (*card).raise();
            }
        }
    }
}
