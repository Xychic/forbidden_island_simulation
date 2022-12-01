use rand::SeedableRng;

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
    dbg!(island_deck);
    dbg!(treasure_deck);
    dbg!(flood_deck);
}
