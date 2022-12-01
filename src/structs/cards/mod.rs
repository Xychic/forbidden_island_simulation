pub mod flood;
pub mod island;
pub mod treasure;

use std::slice::Iter;

use rand::{seq::SliceRandom, Rng};
pub enum CardType {
    Island,
    Treasure,
    Flood,
}

pub trait Card: Clone {
    fn card_type() -> CardType;
}

#[derive(Debug)]
pub struct Deck<A: Card> {
    cards: Vec<A>,
}

#[allow(dead_code)]
impl<A: Card> Deck<A> {
    /// Generates a new Deck of Cards of type `A`
    pub fn new(cards: &[A]) -> Deck<A> {
        Deck {
            cards: cards.to_vec(),
        }
    }

    /// Returns a reference to the element at `index` or `None` if out of bounds
    pub fn peak_card(&self, index: usize) -> Option<&A> {
        self.cards.get(index)
    }

    /// Removes and returns the card at position `index`
    pub fn pop_card(&mut self, index: usize) -> Option<A> {
        if index >= self.cards.len() {
            None
        } else {
            Some(self.cards.remove(index))
        }
    }

    /// Returns a reference to the card on top or `None` if deck is empty
    pub fn peak_next(&self) -> Option<&A> {
        self.peak_card(0)
    }

    /// Removes and returns the card on the top of `self`
    pub fn pop_next(&mut self) -> Option<A> {
        self.pop_card(0)
    }

    /// Adds a card to the bottom of `self`
    pub fn insert(&mut self, card: A) {
        self.cards.push(card);
    }

    /// Moves all cards from `other` to the back of the `self`, leaving `other` empty
    pub fn stack(&mut self, other: &mut Deck<A>) {
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
    pub fn iter(&self) -> Iter<A> {
        self.cards.iter()
    }

    /// Returns the number of elements in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
