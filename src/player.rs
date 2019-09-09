use super::*;
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Player {
    // pub active: usize,
    pub hands: Vec<Hand>,
    pub chips: usize,
}

impl std::ops::Deref for Player {
    type Target = Hand;
    fn deref(&self) -> &Self::Target {
        &self.hands[0]
    }
}

impl std::ops::DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hands[0]
    }
}

impl Player {
    pub fn new(chips: usize) -> Player {
        Player {
            // active: 0,
            hands: vec![Hand::default()],
            chips,
        }
    }

    pub fn can_split(&self, idx: usize) -> bool {
        if self.hands.len() >= 4 {
            return false;
        } else if self.hands.len() > 1 && self.hands[0].cards[0].rank == Rank::Ace {
            return false;
        }

        if let Some(h) = self.hands.get(idx) {
            if h.cards.len() == 2 {
                if h.cards[0].value() == h.cards[1].value() {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    fn initial_hand(&self) -> Option<u8> {
        Some(*self.cards.get(0)? + *self.cards.get(1)?)
    }

    pub fn score(&self) -> u8 {
        let mut score = self.cards.iter().map(|c| c.value()).sum();

        for c in &self.cards {
            if score > 21 && c.rank == Rank::Ace {
                score -= 10;
            }
        }
        score
    }

    pub fn soft(&self) -> bool {
        self.ace_count() >= 1
    }

    pub fn ace_count(&self) -> usize {
        self.cards.iter().filter(|c| c.soft()).count()
    }

    /// Does the player have blackjack?
    pub fn blackjack(&self) -> bool {
        self.initial_hand().unwrap_or(0) == 21
    }

    /// Has the player busted?
    pub fn bust(&self) -> bool {
        self.score() > 21
    }

    pub fn is_splittable(&self) -> bool {
        if self.cards.len() == 2 {
            self.cards[0].value() == self.cards[1].value()
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn deal(&mut self, card: Card) {
        self.cards.push(card)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(Card::notation)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
