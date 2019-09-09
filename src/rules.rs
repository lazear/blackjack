use super::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Ruleset {
    pub decks: usize,
    pub stand: bool,
}

impl Ruleset {
    pub fn default() -> Ruleset {
        Ruleset {
            decks: 1,
            stand: true,
        }
    }

    /// Set the number of decks to be used in the game
    pub fn decks(mut self, decks: usize) -> Ruleset {
        assert!(decks > 0);
        self.decks = decks;
        self
    }

    /// If `stand` is set to true, then the dealer will stand on a soft 17
    pub fn stand(mut self, stand: bool) -> Ruleset {
        self.stand = stand;
        self
    }
}
