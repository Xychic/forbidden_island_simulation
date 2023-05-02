use std::io::{self, Write};

use rand::{Rng, SeedableRng};
use structs::game::moves::Action;

use crate::structs::game::Game;

#[macro_use]
mod structs;

fn main() {
    let mut game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(1), 4, 2, false);
    println!("{:?}", game.play(chooser));
}

fn chooser_2(_stage: Action, v: &Vec<String>) -> usize {
    let mut guess;
    loop {
        // dbg!(&stage);
        for action in v {
            println!("{action}");
        }
        print!("Pick move: ");
        io::stdout().flush().unwrap();
        guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        if let Ok(res) = guess.trim().parse() {
            if res > v.len() {
                println!("[ERROR] number must be less than {}", v.len());
                continue;
            }
            return res;
        }
        println!("[ERROR] Not a number");
    }
}

fn chooser(_stage: Action, v: &Vec<String>) -> usize {
    // for action in v {
    //     println!("{action}");
    // }
    let num = rand::thread_rng().gen_range(0..v.len());
    // println!("{num}");
    num
}
