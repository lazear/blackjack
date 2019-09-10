use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Ruleset {
    pub decks: usize,
    pub stand: bool,
    pub double_after_split: bool,
    pub surrender: bool,
}

impl Ruleset {
    pub fn default() -> Ruleset {
        Ruleset {
            decks: 1,
            stand: true,
            double_after_split: true,
            surrender: false,
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

    pub fn double_after_split(mut self, double_after_split: bool) -> Ruleset {
        self.double_after_split = double_after_split;
        self
    }

    pub fn surrender(mut self, surrender: bool) -> Ruleset {
        self.surrender = surrender;
        self
    }
}
