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

impl std::ops::Deref for Card {
    type Target = Rank;
    fn deref(&self) -> &Self::Target {
        &self.rank
    }
}

impl Card {
    pub fn notation(&self) -> String {
        format!(
            "{}{}",
            match self.rank {
                Two => "2",
                Three => "3",
                Four => "4",
                Five => "5",
                Six => "6",
                Seven => "7",
                Eight => "8",
                Nine => "9",
                Ten => "10",
                Ace => "A",
                King => "K",
                Queen => "Q",
                Jack => "J",
            },
            match self.suit {
                Hearts => 'h',
                Clubs => 'c',
                Spades => 's',
                Diamonds => 'd',
            }
        )
    }
}

impl Rank {
    pub fn is_face(self) -> bool {
        match self {
            Jack | Queen | King | Ace => true,
            _ => false,
        }
    }

    pub fn value(self) -> u8 {
        match self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 10,
            Queen => 10,
            King => 10,
            Ace => 11,
        }
    }

    pub fn soft(self) -> bool {
        match self {
            Ace => true,
            _ => false,
        }
    }
}

impl std::ops::Add for Rank {
    type Output = u8;
    fn add(self, rhs: Self) -> Self::Output {
        self.value() + rhs.value()
    }
}

impl std::ops::Add for Card {
    type Output = u8;
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
