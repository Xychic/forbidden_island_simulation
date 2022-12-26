use rand::SeedableRng;
use structs::game_board::GameBoard;

use crate::structs::cards::{
    adventurer::AdventurerCard,
    flood::FloodCard,
    island::{IslandCard, IslandCardName},
    treasure::TreasureCard,
    Card,
};

#[macro_use]
mod structs;

fn main() {
    // println!("Hello, world!");
    #[allow(non_snake_case)]
    let mut RNGesus = rand_chacha::ChaChaRng::seed_from_u64(1);
    let mut island_deck = IslandCard::get_deck();
    let mut treasure_deck = TreasureCard::get_deck();
    let mut flood_deck = FloodCard::get_deck();
    let mut adventurer_deck = AdventurerCard::get_deck();

    island_deck.shuffle(&mut RNGesus);
    treasure_deck.shuffle(&mut RNGesus);
    flood_deck.shuffle(&mut RNGesus);
    adventurer_deck.shuffle(&mut RNGesus);

    dbg!(adventurer_deck.iter().take(4).collect::<Vec<_>>());

    let mut game_board = GameBoard::new(&mut island_deck);

    game_board.sink(&IslandCardName::CopperGate);
    println!("{}", game_board.show_board());
    dbg!(game_board.get_adjacent(&IslandCardName::DunesOfDeception));
    game_board.shore_up(&IslandCardName::CopperGate);
    // game_board.sink(&IslandCardName::CopperGate);
    dbg!(game_board.get_adjacent(&IslandCardName::DunesOfDeception));
}
