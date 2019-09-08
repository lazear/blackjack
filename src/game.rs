use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    Hit,
    Stand,
    Split,
    Double,
}

pub struct Game {
    deck: Deck,
    dealer: Hand,
    player: Player,
    bet: usize,
    states: Vec<State>,
    active: usize,
    last: Last,
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
    Player,
    Dealer,
    Error,
    Final(Outcome),
}

#[derive(Clone, PartialEq, Debug)]
pub struct View {
    pub dealer: Hand,
    pub player: Player,
    pub states: Vec<State>,
    pub active: usize,
    pub last: Last,
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
        if self.states[self.active] != State::Player {
            return Err(Error::InvalidAction);
        }

        match action {
            Action::Hit => {
                let card = self.draw()?;
                self.last = Last::Player(card);
                self.player.deal(card);
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
                self.player.deal(card);
            }
            Action::Split => {
                if self.player.hands.len() > 1 && self.player.ace_count() > 0 {
                    return Err(Error::InvalidAction);
                }

                if self.player.is_splittable() {
                    let card = self.player.hands[self.player.active]
                        .cards
                        .pop()
                        .ok_or(Error::Fatal)?;
                    self.player.hands.push(Hand { cards: vec![card] });
                    self.states.push(State::Player);
                }
            }
        }

        self.refresh();
        Ok(self.view())
    }

    pub fn view(&self) -> View {
        let dealer = match self.states[self.active] {
            State::Final(_) | State::Dealer => self.dealer.clone(),
            _ => Hand {
                cards: self.dealer.cards[1..].to_vec(),
            },
        };
        View {
            dealer,
            player: self.player.clone(),
            states: self.states.clone(),
            active: self.active,
            last: self.last,
        }
    }

    /// Initialize a game to the Ready state
    pub fn init(player: Player) -> Game {
        Game {
            deck: Deck::new(),
            dealer: Hand::default(),
            player,
            bet: 0,
            states: vec![State::Ready],
            active: 0,
            last: Last::Player(Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }),
        }
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
                self.states[self.active] = State::Error;
                self.bet = 0;
                Err(Error::Fatal)
            }
        }
    }

    pub fn test(&mut self) {
        self.player.hands = vec![Hand {
            cards: vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                },
            ],
        }];
    }

    /// Once the game is in Ready state, the player may place a bet and be
    /// dealt a hand of cards
    pub fn bet(&mut self, bet: usize) -> Result<View, Error> {
        // Don't let the player set their starting hand!
        // self.player.hands = vec![Hand::default()];
        // self.player.active = 0;

        if self.states[self.active] != State::Ready {
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
            self.states[self.active] = State::Player;

            // Check for initial blackjack
            self.refresh();
            Ok(self.view())
        }
    }

    fn blackjack_amount(&self) -> usize {
        (self.bet as f64 * 2.5).floor() as usize
    }

    fn update_state(&mut self, state: State) {
        self.states[self.active] = state;
        if self.active < self.states.len() - 1 {
            self.active += 1;
        }
    }

    fn refresh(&mut self) {
        match self.states[self.active] {
            State::Final(_) | State::Dealer | State::Error | State::Ready => return,
            _ => {}
        }
        self.player.active = self.active;
        if self.player.blackjack() {
            if self.dealer.blackjack() {
                self.update_state(State::Final(Outcome::Push(self.bet)));
            } else {
                self.update_state(State::Final(Outcome::Blackjack(self.blackjack_amount())));
            }
        }

        if self.player.bust() {
            self.update_state(State::Final(Outcome::Lose(self.bet)));
        }
    }

    pub fn update(&mut self) -> Result<View, Error> {
        if self.states.iter().filter(|s| *s == &State::Dealer).count() < 1 {
            return Err(Error::InvalidAction);
        }

        if self.dealer.score() < 17 {
            let card = self.draw()?;
            self.last = Last::Dealer(card);
            self.dealer.deal(card);
        }

        for i in 0..self.states.len() {
            if self.dealer.blackjack() {
                if let State::Final(Outcome::Blackjack(_)) = self.states[i] {
                    self.states[i] = State::Final(Outcome::Push(self.bet));
                }
            }

            if self.states[i] != State::Dealer {
                continue;
            }

            // if self.dealer.blackjack() {
            //     self.states[i] = State::Final(Outcome::Lose(self.bet));
            //     return Ok(self.view());
            // }

            // else if self.dealer.score() >= 17 {
            //     let outcome = if self.player.score() > 17 {
            //         Outcome::Win(self.bet * 2)
            //     } else if self.player.score() == 17 {
            //         Outcome::Push(self.bet)
            //     } else {
            //         Outcome::Lose(self.bet)
            //     };
            //     self.states[self.active] = State::Final(outcome)
            // }

            // We have now possibly drawn a card for the dealer, so check to see
            // if we have beaten the player
            if self.dealer.bust() {
                self.states[i] = State::Final(Outcome::Win(self.bet * 2))
            } else if self.dealer.score() > self.player.hands[i].score() {
                self.states[i] = State::Final(Outcome::Lose(self.bet))
            } else if self.dealer.score() == self.player.hands[i].score() {
                self.states[i] = State::Final(Outcome::Push(self.bet))
            }
        }

        Ok(self.view())
    }

    /// Winnings are not transferred back to the player until finish()
    /// is called. This forces the round to go to completion
    pub fn finish(mut self) -> Result<Player, Error> {
        for i in 0..self.states.len() {
            match self.states[i] {
                State::Final(fin) => match fin {
                    Outcome::Blackjack(win) => self.player.chips += win,
                    Outcome::Win(win) => self.player.chips += win,
                    Outcome::Push(win) => self.player.chips += win,
                    _ => {}
                },
                _ => return Err(Error::InvalidAction),
            }
        }

        Ok(self.player)
    }
}
