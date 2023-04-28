use rand::SeedableRng;

use crate::structs::game::Game;

#[macro_use]
mod structs;

fn main() {
    let game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(1), 4, 2);

    dbg!(&game);
    println!("{}", game.board.show());
}
