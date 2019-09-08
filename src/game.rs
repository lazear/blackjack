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
    state: State,
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
    PlayerSplit,
    Dealer,
    Error,
    Final(Outcome),
}

#[derive(Clone, PartialEq, Debug)]
pub struct View {
    pub dealer: Hand,
    pub player: Player,
    pub state: State,
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
        if self.state != State::Player {
            return Err(Error::InvalidAction);
        }

        match action {
            Action::Hit => {
                let card = self.draw()?;
                self.last = Last::Player(card);
                self.player.deal(card);
            }
            Action::Stand => {
                self.state = State::Dealer;
            }
            Action::Double => {
                if self.player.chips < self.bet {
                    return Err(Error::Money(self.bet - self.player.chips));
                }
                self.state = State::Dealer;
                self.player.chips -= self.bet;
                self.bet *= self.bet;

                let card = self.draw()?;
                self.last = Last::Player(card);
                self.player.deal(card);
            }
            Action::Split => {}
        }

        self.refresh();
        Ok(self.view())
    }

    pub fn view(&self) -> View {
        let dealer = match self.state {
            State::Final(_) | State::Dealer => self.dealer.clone(),
            _ => Hand {
                cards: self.dealer.cards[1..].to_vec(),
            },
        };
        View {
            dealer,
            player: self.player.clone(),
            state: self.state,
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
            state: State::Ready,
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
        self.player.hand = Hand::default();

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
            self.state = State::Player;

            // Check for initial blackjack
            self.refresh();
            Ok(self.view())
        }
    }

    fn blackjack_amount(&self) -> usize {
        (self.bet as f64 * 2.5).floor() as usize
    }

    fn refresh(&mut self) {
        match self.state {
            State::Final(_) | State::Dealer | State::Error | State::Ready => return,
            _ => {}
        }

        if self.player.blackjack() {
            if self.dealer.blackjack() {
                self.state = State::Final(Outcome::Push(self.bet));
            } else {
                self.state = State::Final(Outcome::Blackjack(self.blackjack_amount()));
            }
        }

        if self.player.bust() {
            self.state = State::Final(Outcome::Lose(self.bet))
        }
    }

    /// Winnings are not transferred back to the player until finalize()
    /// is called. This forces the round to go to completion
    pub fn update(&mut self) -> Result<View, Error> {
        if self.state != State::Dealer {
            return Err(Error::InvalidAction);
        }

        if self.dealer.blackjack() {
            self.state = State::Final(Outcome::Lose(self.bet));
            return Ok(self.view());
        } else if self.dealer.score() == 17 && self.dealer.soft() {
            let outcome = if self.player.score() > 17 {
                Outcome::Win(self.bet * 2)
            } else if self.player.score() == 17 {
                Outcome::Push(self.bet)
            } else {
                Outcome::Lose(self.bet)
            };
            self.state = State::Final(outcome)
        } else if self.dealer.score() < self.player.score() {
            let card = self.draw()?;
            self.last = Last::Dealer(card);
            println!("Dealer draws a {}", &card);
            self.dealer.deal(card);
        }

        // We have now possibly drawn a card for the dealer, so check to see
        // if we have beaten the player
        if self.dealer.bust() {
            println!("Dealer went bust {}", self.dealer.score());
            self.state = State::Final(Outcome::Win(self.bet * 2))
        } else if self.dealer.score() > self.player.score() {
            self.state = State::Final(Outcome::Lose(self.bet))
        } else if self.dealer.score() == self.player.score() {
            self.state = State::Final(Outcome::Push(self.bet))
        }

        Ok(self.view())
    }

    pub fn finish(mut self) -> Result<Player, Error> {
        match self.state {
            State::Final(fin) => match fin {
                Outcome::Blackjack(win) => self.player.chips += win,
                Outcome::Win(win) => self.player.chips += win,
                Outcome::Push(win) => self.player.chips += win,
                _ => {}
            },
            _ => return Err(Error::InvalidAction),
        }

        Ok(self.player)
    }
}
