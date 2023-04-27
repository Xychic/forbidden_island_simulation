use rand::SeedableRng;

use crate::structs::{cards::island::IslandCardName, game::Game};

#[macro_use]
mod structs;

fn main() {
    let mut game = Game::<4, _>::new(rand_chacha::ChaChaRng::seed_from_u64(1));

    dbg!(game.adventurers);

    game.board.sink(&IslandCardName::CopperGate);
    println!("{}", game.board.show());
    dbg!(game.board.get_adjacent(&IslandCardName::DunesOfDeception));
    game.board.shore_up(&IslandCardName::CopperGate);
    // game.board.sink(&IslandCardName::CopperGate);
    dbg!(game.board.get_adjacent(&IslandCardName::DunesOfDeception));
}
