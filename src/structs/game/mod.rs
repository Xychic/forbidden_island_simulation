use rand::Rng;

use super::{
    board::Board,
    cards::{
        adventurer::AdventurerCard, flood::FloodCard, island::IslandCard, treasure::TreasureCard,
        Card, Deck,
    },
};

pub struct Game<const N: usize, R: Rng> {
    pub rng: R,
    pub island_deck: Deck<IslandCard>,
    pub treasure_deck: Deck<TreasureCard>,
    pub flood_deck: Deck<FloodCard>,
    pub adventurers: [AdventurerCard; N],
    pub board: Board,
}

impl<const N: usize, R: Rng> Game<N, R> {
    pub fn new(mut rng: R) -> Game<N, R> {
        let mut island_deck = IslandCard::get_deck();
        let mut treasure_deck = TreasureCard::get_deck();
        let mut flood_deck = FloodCard::get_deck();
        let mut adventurer_deck = AdventurerCard::get_deck();

        // Shuffle the decks
        island_deck.shuffle(&mut rng);
        treasure_deck.shuffle(&mut rng);
        flood_deck.shuffle(&mut rng);
        adventurer_deck.shuffle(&mut rng);

        let board = Board::new(&mut island_deck);

        Game {
            rng,
            island_deck,
            treasure_deck,
            flood_deck,
            adventurers: (0..N)
                .map(|_| adventurer_deck.pop_next().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            board,
        }
    }
}
