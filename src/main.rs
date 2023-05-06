use std::{
    fmt::Debug,
    io::{self, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use rand::{Rng, SeedableRng};
use structs::game::moves::Action;
use thousands::Separable;

use crate::structs::game::Game;

#[macro_use]
mod structs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of games to play
    #[arg(short, long, default_value_t = 100_000)]
    games: u64,

    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: u64,
}

fn main() {
    let args = Cli::parse();
    let thread_count = args.threads;
    let simulation_count = args.games;
    let (tx, rx) = mpsc::channel();

    let start = Instant::now();
    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let thread_tx = tx.clone();
            thread::spawn(move || {
                let range = simulation_count / thread_count;
                let lower = range * i;
                let upper = if i == thread_count - 1 {
                    simulation_count
                } else {
                    range * (i + 1)
                };

                let mut timer = Instant::now();
                let mut rounds_simulated = 0;
                let mut games_simulated = 0;
                let mut max_game_length = 0;

                for rand_seed in lower..upper {
                    let mut game = Game::new(
                        rand_chacha::ChaChaRng::seed_from_u64(rand_seed),
                        4,
                        2,
                        false,
                    );
                    let (_, _, rounds) = game.play(chooser);
                    games_simulated += 1;
                    rounds_simulated += rounds;
                    max_game_length = max_game_length.max(rounds);
                    if (Instant::now() - timer).as_millis() >= 1000 {
                        thread_tx
                            .send((rounds_simulated, games_simulated, max_game_length))
                            .unwrap();
                        rounds_simulated = 0;
                        games_simulated = 0;
                        timer = Instant::now();
                    }
                }
                (rounds_simulated, games_simulated, max_game_length)
            })
        })
        .collect();

    let mut rounds_simulated = 0;
    let mut games_simulated = 0;
    let mut max_game_length = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        while let Ok((r, g, m)) = rx.try_recv() {
            rounds_simulated += r;
            games_simulated += g;
            max_game_length = max_game_length.max(m);
        }

        print!(
            "Rate: {} rounds/s\t{}\t{max_game_length}\r",
            (rounds_simulated / (Instant::now() - start).as_secs()).separate_with_commas(),
            games_simulated.separate_with_commas()
        );
        io::stdout().flush().unwrap();

        if handles.iter().all(|h| h.is_finished()) {
            for h in handles {
                let (r, g, m) = h.join().unwrap();
                rounds_simulated += r;
                games_simulated += g;
                max_game_length = max_game_length.max(m);
            }
            println!(
                "Rate: {} rounds/s\t{}\t{max_game_length}\r",
                (rounds_simulated / (Instant::now() - start).as_secs()).separate_with_commas(),
                games_simulated.separate_with_commas()
            );
            break;
        }
    }
}

fn _manual_chooser<T: Rng>(_data: &Game<T>, _stage: Action, v: &Vec<String>) -> usize {
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

fn chooser<T: Rng + Debug>(_data: &Game<T>, stage: Action, v: &Vec<String>) -> usize {
    if v.is_empty() {
        dbg!(stage);
        panic!()
    }
    rand::thread_rng().gen_range(0..v.len())
}
