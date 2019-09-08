use Rank::*;
use Suit::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Value {
    Hard(u8),
    Soft(u8),
}

impl Card {
    pub fn value(self) -> Value {
        self.rank.value()
    }
}

impl Rank {
    pub fn is_face(self) -> bool {
        match self {
            Jack | Queen | King | Ace => true,
            _ => false,
        }
    }

    pub fn value(self) -> Value {
        match self {
            Two => Value::Hard(2),
            Three => Value::Hard(3),
            Four => Value::Hard(4),
            Five => Value::Hard(5),
            Six => Value::Hard(6),
            Seven => Value::Hard(7),
            Eight => Value::Hard(8),
            Nine => Value::Hard(9),
            Ten => Value::Hard(10),
            Jack => Value::Hard(10),
            Queen => Value::Hard(10),
            King => Value::Hard(10),
            Ace => Value::Soft(11),
        }
    }
}

impl Value {
    pub fn busted(self) -> bool {
        self.min() > 21
    }

    pub fn max(self) -> u8 {
        match self {
            Value::Hard(val) => val,
            Value::Soft(val) => val,
        }
    }

    pub fn min(self) -> u8 {
        match self {
            Value::Hard(val) => val,
            Value::Soft(val) => val - 10,
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Hard(a), Value::Hard(b)) => Value::Hard(a + b),
            (Value::Hard(a), Value::Soft(b)) => Value::Soft(a + b),
            (Value::Soft(a), Value::Hard(b)) => Value::Soft(a + b),
            (Value::Soft(a), Value::Soft(b)) => Value::Soft(a + b),
        }
    }
}

impl std::ops::Add for Rank {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        self.value() + rhs.value()
    }
}

impl std::ops::Add for Card {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        self.rank + rhs.rank
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

mod tests {
    use super::*;

    #[test]
    fn add_cards() {
        let a = Two;
        let b = Ace;
        assert_eq!(a + b, Value::Soft(13));
    }
}
