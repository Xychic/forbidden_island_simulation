use rand::SeedableRng;

use crate::structs::{cards::adventurer::AdventurerCardType, game::Game};

#[macro_use]
mod structs;

fn main() {
    let mut game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(1), 4, 2);

    dbg!(&game);
    dbg!(&game.adventurers);

    println!("{}", &game.board.show());

    game.do_action(&AdventurerCardType::Navigator, |v| {
        dbg!(v);

        4
    });

    dbg!(&game.adventurers);

    // game.get_actions(game.adventurers.get(&AdventurerCardType::Pilot).unwrap())
}
