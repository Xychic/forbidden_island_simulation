use rand::SeedableRng;
use structs::game_board::GameBoard;

use crate::structs::cards::island::IslandCardName;

mod decks;
mod structs;

fn main() {
    // println!("Hello, world!");
    #[allow(non_snake_case)]
    let mut RNGesus = rand_chacha::ChaChaRng::seed_from_u64(1);
    let mut island_deck = decks::get_island_deck();
    let mut treasure_deck = decks::get_treasure_deck();
    let mut flood_deck = decks::get_flood_deck();

    island_deck.shuffle(&mut RNGesus);
    treasure_deck.shuffle(&mut RNGesus);
    flood_deck.shuffle(&mut RNGesus);
    // dbg!(island_deck);
    // dbg!(treasure_deck);
    // dbg!(flood_deck);

    let mut game_board = GameBoard::new(&mut island_deck);

    println!("{}", game_board.show_board());
    game_board.sink(&IslandCardName::CrimsonForest);
    // dbg!(game_board);
    println!("{}", game_board.show_board());
    game_board.shore_up(&IslandCardName::CrimsonForest);
    println!("{}", game_board.show_board());
    game_board.sink(&IslandCardName::CrimsonForest);
    game_board.sink(&IslandCardName::CrimsonForest);
    println!("{}", game_board.show_board());
}
