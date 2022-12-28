use rand::SeedableRng;
use structs::game_board::GameBoard;

use crate::structs::cards::{
    adventurer::{AdventurerCard, AdventurerCardType},
    flood::FloodCard,
    island::{IslandCard, IslandCardName},
    treasure::{TreasureCard, TreasureCardType, TreasureType},
    Card,
};

#[macro_use]
mod structs;

fn main() {
    // println!("Hello, world!");
    let mut game_board = GameBoard::new(&mut rand_chacha::ChaChaRng::seed_from_u64(1), 2);

    game_board.sink(&IslandCardName::CopperGate);
    game_board
        .adventurer_locations
        .entry(AdventurerCardType::Diver)
        .and_modify(|(a, pos)| {
            a.receive_card(TreasureCard::new(&TreasureCardType::Treasure(
                TreasureType::Earth,
            )));
            a.receive_card(TreasureCard::new(&TreasureCardType::Treasure(
                TreasureType::Earth,
            )));
            a.receive_card(TreasureCard::new(&TreasureCardType::Treasure(
                TreasureType::Earth,
            )));
            a.receive_card(TreasureCard::new(&TreasureCardType::Treasure(
                TreasureType::Earth,
            )));
            *pos = (2, 3);
        });
    println!("{}", game_board.show_board());

    dbg!(game_board.get_options(&AdventurerCardType::Diver, 3));
    // dbg!(game_board.get_moves(&AdventurerCardType::Pilot));
    // dbg!(game_board.get_adjacent(&IslandCardName::DunesOfDeception));
    // game_board.shore_up(&IslandCardName::CopperGate);
    // // game_board.sink(&IslandCardName::CopperGate);
    // dbg!(game_board.get_adjacent(&IslandCardName::DunesOfDeception));
}
