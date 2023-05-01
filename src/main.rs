use std::io::{self, Write};

use rand::SeedableRng;

use crate::structs::{
    cards::{
        adventurer::AdventurerCardType,
        treasure::{TreasureCard, TreasureCardType, TreasureType},
    },
    game::Game,
};

#[macro_use]
mod structs;

fn main() {
    let mut game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(1), 4, 2);

    dbg!(&game);

    println!("{}", &game.board.show());

    game.do_action(&AdventurerCardType::Pilot, chooser);
}

fn chooser(v: &Vec<String>) -> usize {
    dbg!(v);

    print!("Pick move: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess.trim().parse().unwrap()
}
