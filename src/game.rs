//! The `game` module represents the Blackjack game engine and logic
//!
//! A custom random number generator can be supplied, for instance, to always
//! deal the same hands (with a deterministicly seeded PRNG) in the same order
use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    Hit,
    Stand,
    Split,
    Double,
}

pub struct Game {
    rules: Ruleset,
    deck: Deck,
    dealer: Hand,
    player: Player,
    bet: usize,
    state: State,
    last: Last,
    scores: Vec<Outcome>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Last {
    Dealer(Card),
    Player(Card),
}

/// Player's states.
/// Loss = player's loss, etc
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Outcome {
    Lose(usize),
    Win(usize),
    Blackjack(usize),
    Push(usize),
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum State {
    Ready,
    Player(usize),
    Dealer,
    Error,
    Final,
}

#[derive(Clone, PartialEq, Debug)]
pub struct View {
    pub dealer: Hand,
    pub player: Player,
    pub state: State,
    pub last: Last,
    pub scores: Vec<Outcome>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Error {
    InvalidAction,
    Final,
    Fatal,
    Money(usize),
}

impl Game {
    pub fn action(&mut self, action: Action) -> Result<View, Error> {
        let hidx;
        match self.state {
            State::Player(idx) => {
                hidx = idx;
            }
            _ => return Err(Error::InvalidAction),
        }

        match action {
            Action::Hit => {
                let card = self.draw()?;
                self.last = Last::Player(card);
                self.player.hands[hidx].deal(card);
            }
            Action::Stand => {
                self.update_state(State::Dealer);
            }
            Action::Double => {
                if self.player.chips < self.bet {
                    return Err(Error::Money(self.bet - self.player.chips));
                }
                self.update_state(State::Dealer);
                self.player.chips -= self.bet;
                self.bet += self.bet;

                let card = self.draw()?;
                self.last = Last::Player(card);
                self.player.hands[hidx].deal(card);
            }
            Action::Split => {
                if self.player.can_split(hidx) {
                    self.player.chips -= self.bet;

                    let split = self.player.hands[hidx].cards.pop().ok_or(Error::Fatal)?;

                    let card = self.draw()?;
                    self.player.hands[hidx].deal(card);

                    let card = self.draw()?;
                    self.last = Last::Player(card);
                    self.player.hands.push(Hand {
                        cards: vec![split, card],
                    });
                } else {
                    return Err(Error::InvalidAction);
                }
            }
        }

        self.refresh();
        Ok(self.view())
    }

    pub fn view(&self) -> View {
        let dealer = match self.state {
            State::Dealer | State::Final => self.dealer.clone(),
            _ => Hand {
                cards: self.dealer.cards[1..].to_vec(),
            },
        };

        View {
            dealer,
            player: self.player.clone(),
            state: self.state,
            last: self.last,
            scores: self.scores.clone(),
        }
    }

    /// Initialize a game to the Ready state
    pub fn init<R: rand::Rng>(rules: Ruleset, player: Player, rng: &mut R) -> Game {
        let mut g = Game {
            rules,
            deck: Deck::new(6),
            dealer: Hand::default(),
            player,
            bet: 0,
            state: State::Ready,
            last: Last::Player(Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }),
            scores: Vec::new(),
        };
        g.deck.shuffle(rng);
        g
    }

    /// Deal cards to all players
    fn deal(&mut self) -> Result<(), Error> {
        for _ in 0..2 {
            let c = self.draw()?;
            self.player.deal(c);
            let c = self.draw()?;
            self.dealer.deal(c);
            self.last = Last::Dealer(c);
        }
        Ok(())
    }

    /// Draw a card from the deck
    fn draw(&mut self) -> Result<Card, Error> {
        match self.deck.draw() {
            Some(card) => Ok(card),
            None => {
                self.player.chips += self.bet;
                self.state = State::Error;
                self.bet = 0;
                Err(Error::Fatal)
            }
        }
    }

    /// Once the game is in Ready state, the player may place a bet and be
    /// dealt a hand of cards
    pub fn bet(&mut self, bet: usize) -> Result<View, Error> {
        // Don't let the player set their starting hand!
        self.player.hands = vec![Hand::default()];
        // self.player.active = 0;

        if self.state != State::Ready {
            return Err(Error::InvalidAction);
        }
        if bet == 0 {
            Err(Error::InvalidAction)
        } else if self.player.chips < bet {
            Err(Error::Money(bet - self.player.chips))
        } else {
            assert_eq!(self.player.count(), 0);
            self.deal()?;
            assert_eq!(self.player.count(), 2);
            self.player.chips -= bet;
            self.bet += bet;
            self.state = State::Player(0);

            // Check for initial blackjack
            self.refresh();
            Ok(self.view())
        }
    }

    fn blackjack_amount(&self) -> usize {
        (self.bet as f64 * 2.5).floor() as usize
    }

    fn update_state(&mut self, state: State) {
        match self.state {
            State::Player(idx) if idx < self.player.hands.len() - 1 => {
                self.state = State::Player(idx + 1)
            }
            _ => self.state = state,
        }
    }

    fn refresh(&mut self) {
        if let State::Player(hidx) = self.state {
            if self.player.hands[hidx].blackjack() || self.player.hands[hidx].bust() {
                self.update_state(State::Dealer);
            }
        }
    }

    pub fn dealer(&mut self) -> Result<View, Error> {
        if self.state != State::Dealer {
            return Err(Error::InvalidAction);
        }

        // If there's a hand that isn't busted (i.e. player stood or got blackjack)
        // then we will continue to draw cards to try and beat them
        for hidx in 0..self.player.hands.len() {
            if !self.player.hands[hidx].bust() {
                if self.dealer.score() < 17
                    || (self.dealer.score() == 17 && self.dealer.soft() && !self.rules.stand)
                {
                    let card = self.draw()?;
                    self.last = Last::Dealer(card);
                    self.dealer.deal(card);
                    if !self.dealer.bust() && !self.dealer.blackjack() {
                        return Ok(self.view());
                    }
                }
            }
        }

        // We have now possibly drawn cards for the dealer, so check to see
        // if we have beaten the player
        for hand in &self.player.hands {
            if hand.bust() {
                self.scores.push(Outcome::Lose(self.bet));
            } else if hand.blackjack() {
                if self.dealer.blackjack() {
                    self.scores.push(Outcome::Push(self.bet));
                } else {
                    self.scores
                        .push(Outcome::Blackjack(self.blackjack_amount()));
                }
            } else if self.dealer.score() > hand.score() && !self.dealer.bust() {
                self.scores.push(Outcome::Lose(self.bet));
            } else if self.dealer.score() == hand.score() {
                self.scores.push(Outcome::Push(self.bet));
            } else if self.dealer.score() < hand.score() {
                self.scores.push(Outcome::Win(self.bet * 2));
            } else {
                self.scores.push(Outcome::Win(self.bet * 2));
            }
        }
        self.state = State::Final;
        Ok(self.view())
    }

    /// Winnings are not transferred back to the player until finish()
    /// is called. This forces the round to go to completion
    pub fn finish(mut self) -> Result<Player, Error> {
        if self.state != State::Final {
            return Err(Error::InvalidAction);
        }
        for score in self.scores {
            match score {
                Outcome::Blackjack(win) => self.player.chips += win,
                Outcome::Win(win) => self.player.chips += win,
                Outcome::Push(win) => self.player.chips += win,
                _ => {}
            }
        }
        Ok(self.player)
    }
}
