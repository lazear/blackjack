use super::card::{Card, Rank::*, Suit::*};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Initialize a new deck that has been shuffled once
    pub fn new(count: usize) -> Deck {
        let suits = vec![Hearts, Spades, Clubs, Diamonds];
        let ranks = vec![
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];

        let mut deck = Deck { cards: Vec::new() };
        for _ in 0..count {
            for suit in &suits {
                for rank in &ranks {
                    deck.cards.push(Card {
                        rank: *rank,
                        suit: *suit,
                    });
                }
            }
        }
        deck
    }

    /// Fisher-Yates shuffling algorithim
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        let n = self.cards.len();
        for i in (0..n).rev() {
            self.cards.swap(i, rng.gen_range(0, n));
        }
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn notation(&self) -> String {
        self.cards
            .iter()
            .map(Card::notation)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
