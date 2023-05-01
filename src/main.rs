use std::io::{self, Write};

use rand::SeedableRng;
use structs::game::moves::ActionStage;

use crate::structs::{cards::adventurer::AdventurerCardType, game::Game};

#[macro_use]
mod structs;

fn main() {
    let mut game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(1), 4, 2);

    dbg!(&game);

    println!("{}", &game.board.show());

    let pos = game
        .get_adventurer(&AdventurerCardType::Engineer)
        .unwrap()
        .pos;
    game.get_adventurer_mut(&AdventurerCardType::Diver)
        .unwrap()
        .pos = pos;
    game.intial_actions(&AdventurerCardType::Engineer, chooser_2, 3);
}

fn chooser_2(stage: ActionStage, v: &Vec<String>) -> usize {
    dbg!(stage);
    dbg!(v);

    print!("Pick move: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess.trim().parse().unwrap()
}
