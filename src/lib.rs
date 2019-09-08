pub mod card;
pub mod deck;
pub mod game;
pub mod player;

pub use card::*;
use deck::Deck;
pub use game::{Action, Game, View};
pub use player::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
