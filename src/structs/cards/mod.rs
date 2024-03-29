pub mod adventurer;
pub mod flood;
pub mod island;
pub mod treasure;

use std::slice::Iter;

use rand::{seq::SliceRandom, Rng};
pub enum CardType {
    Island,
    Treasure,
    Flood,
    Adventurer,
}

pub trait Card: Clone {
    fn card_type() -> CardType;
    fn get_deck() -> Deck<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deck<A: Card> {
    pub cards: Vec<A>,
}

#[allow(dead_code)]
impl<T: Card> Deck<T> {
    /// Copies `cards` into a new `Deck<T>`.
    pub fn from(cards: &[T]) -> Deck<T> {
        Deck {
            cards: cards.to_vec(),
        }
    }

    /// Constructs a new, empty `Deck<T>` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Deck<T> {
        Deck {
            cards: Vec::with_capacity(capacity),
        }
    }

    /// Constructs a new, empty `Deck<T>`.
    pub fn new() -> Deck<T> {
        Deck { cards: Vec::new() }
    }

    /// Returns a reference to the element at `index` or `None` if out of bounds
    pub fn peak_card(&self, index: usize) -> Option<&T> {
        self.cards.get(index)
    }

    /// Removes and returns the card at position `index`
    pub fn pop_card(&mut self, index: usize) -> Option<T> {
        if index >= self.cards.len() {
            None
        } else {
            Some(self.cards.remove(index))
        }
    }

    /// Returns a reference to the card on top or `None` if deck is empty
    pub fn peak_next(&self) -> Option<&T> {
        self.peak_card(0)
    }

    /// Removes and returns the card on the top of `self`
    pub fn pop_next(&mut self) -> Option<T> {
        self.pop_card(0)
    }

    /// Adds a card to the bottom of `self`
    pub fn insert(&mut self, card: T) {
        self.cards.push(card);
    }

    /// Moves all cards from `other` to the back of the `self`, leaving `other` empty
    pub fn stack(&mut self, other: &mut Deck<T>) {
        self.cards.append(&mut other.cards);
    }

    /// Shuffles `self` in place
    pub fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng,
    {
        self.cards.shuffle(rng);
    }

    /// Returns an iterator over the deck of cards
    pub fn iter(&self) -> Iter<T> {
        self.cards.iter()
    }

    /// Returns the number of elements in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `true` if the deck contains no cards
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[macro_export]
macro_rules! deck {
    () => {
        Deck::new()
    };
    ($($x:expr),+ $(,)?) => {
        Deck {
            cards: vec![$($x),+]
        }
    };
}
