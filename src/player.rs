use super::*;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Player {
    pub hand: Hand,
    pub chips: usize,
}

impl std::ops::Deref for Player {
    type Target = Hand;
    fn deref(&self) -> &Self::Target {
        &self.hand
    }
}

impl std::ops::DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hand
    }
}

impl Player {
    pub fn new(chips: usize) -> Player {
        Player {
            hand: Hand::default(),
            chips,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn first(&self) -> Option<&Card> {
        self.cards.get(0)
    }

    pub fn second(&self) -> Option<&Card> {
        self.cards.get(1)
    }

    pub fn initial_hand(&self) -> Option<Value> {
        Some(*self.cards.get(0)? + *self.cards.get(1)?)
    }

    pub fn score(&self) -> Value {
        self.cards
            .iter()
            .fold(Value::Hard(0), |acc, x| acc + x.value())
    }

    pub fn blackjack(&self) -> bool {
        self.initial_hand().unwrap_or(Value::Hard(0)) == Value::Soft(21)
    }

    pub fn bust(&self) -> bool {
        self.score().busted()
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

    pub fn soft(&self) -> bool {
        let mut soft = false;
        for c in &self.cards {
            if c.rank == Rank::Ace {
                soft = true;
            }
        }
        soft
    }
}
