use std::{
    io::{self, Write},
    time::Instant,
};

use rand::{Rng, SeedableRng};
use structs::game::moves::Action;
use thousands::Separable;

use crate::structs::game::Game;

#[macro_use]
mod structs;

fn main() {
    let mut timer = Instant::now();
    let start = timer;
    let mut rounds_simulated = 0;
    let mut wins = 0;

    for i in 0..10_000_000 {
        let mut game = Game::new(rand_chacha::ChaChaRng::seed_from_u64(i), 4, 2, false);
        let (win, _, rounds) = game.play(chooser);
        if win {
            wins += 1;
        }
        rounds_simulated += rounds;
        if (Instant::now() - timer).as_millis() >= 1000 {
            print!(
                "{i} Rate: {} rounds/s\r",
                (rounds_simulated / (Instant::now() - start).as_secs()).separate_with_commas()
            );
            io::stdout().flush().unwrap();
            timer = Instant::now();
        }
    }
    println!(
        "Rate: {} rounds/s",
        (rounds_simulated / (Instant::now() - start).as_secs()).separate_with_commas()
    );
    println!("{wins} wins");
}

fn _manual_chooser(_stage: Action, v: &Vec<String>) -> usize {
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

fn chooser(stage: Action, v: &Vec<String>) -> usize {
    if v.is_empty() {
        dbg!(stage);
        panic!()
    }
    rand::thread_rng().gen_range(0..v.len())
}
