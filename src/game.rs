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
}

/// Player's states.
/// Loss = player's loss, etc
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Outcome {
    Lose(usize),
    Win(usize),
    Blackjack(usize),
    Push(usize),
    Continue,
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
pub struct View<'a> {
    pub dealer: Hand,
    pub player: &'a Player,
    pub state: State,
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
                self.player.deal(card);
            }
            Action::Stand => {
                self.state = State::Dealer;
            }
            Action::Double => {
                if self.player.chips < self.bet {
                    return Err(Error::Money(self.bet - self.player.chips))
                }
                self.state = State::Dealer;
                self.player.chips -= self.bet;
                self.bet *= self.bet;  

                let card = self.draw()?;
                self.player.deal(card);
            }
            Action::Split => {},
        }

        self.refresh();        
        Ok(self.view())
    }

    pub fn view(&self) -> View {
        View {
            dealer: Hand { cards: self.dealer.cards[1..].to_vec() },
            player: &self.player,
            state: self.state,
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
        }
    }
    
    /// Deal cards to all players
    fn deal(&mut self) -> Result<(), Error> {
        for _ in 0..2 {
            let c = self.draw()?;
            self.player.deal(c);
            let c = self.draw()?;
            self.dealer.deal(c);
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
        if self.state != State::Ready {
            return Err(Error::InvalidAction)
        }
        if bet == 0 {
            Err(Error::InvalidAction)
        } else if self.player.chips < bet {
            Err(Error::Money(bet - self.player.chips))
        } else {
            self.deal()?;
            self.player.chips -= bet;
            self.bet += bet;            
            self.state = State::Player;

            // Check for initial blackjack
            self.refresh();                      
            Ok(self.view())       
        }
    }

    fn blackjack_amount(&self) -> usize {
        (self.bet as f64 * 1.5).floor() as usize
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
            return Err(Error::InvalidAction)
        }
        
        if self.dealer.blackjack() {
            self.state = State::Final(Outcome::Lose(self.bet));
            return Ok(self.view())
        }

        
        if self.dealer.score() == 17 && self.dealer.soft() {
            
        }

        Ok(self.view())
    }
}

// impl Game {

//     fn deal_to_all(&mut self) {
//         for _ in 0..2 {
//             self.player.deal(self.deck.draw().unwrap());
//             self.dealer.deal(self.deck.draw().unwrap());
//         }
//         assert_eq!(self.player.count(), 2);
//         assert_eq!(self.dealer.count(), 2);
//     }

    

//     pub fn play<F>(&mut self, bet: usize, player_action: F) -> State
//     where
//         F: Fn(View) -> Action,
//     {
//         self.deal_to_all();
//         self.bet = bet;

//         loop {
//             if self.player.bust() {
//                 return State::Lose(self.bet);
//             }
//             if self.player.blackjack() && !self.dealer.blackjack() {
//                 return State::Blackjack((self.bet as f64 * 1.5).floor() as usize);
//             }
//             if self.player.blackjack() && self.dealer.blackjack() {
//                 return State::Push(self.bet);
//             }

//             if self.player.hand.is_splittable() {
//                 self.available_actions.push(Action::Split);
//             }

//             match player_action(self.view()) {
//                 Action::Double => {
//                     if self.player.chips >= bet {
//                         // No more actions after doubling down
//                         self.available_actions = Vec::new();
//                         self.bet += bet;
//                         self.player.deal(self.deck.draw().unwrap());
//                         break;
//                     }
//                 }
//                 Action::Hit => {
//                     self.player.deal(self.deck.draw().unwrap());
//                 }
//                 Action::Split => {}
//                 Action::Stand => {
//                     self.available_actions = Vec::new();
//                     break;
//                 }
//             }
//         }

//         println!(
//             "Dealer reveal: {:?} {:?}",
//             self.dealer.cards,
//             self.dealer.score()
//         );
//         loop {
//             match self.dealer.score() {
//                 Value::Hard(val) => {
//                     if val < self.player.score().max() {
//                         let card = self.deck.draw().unwrap();
//                         println!("Dealer draws {}", &card);
//                         self.dealer.deal(card);
//                     } else {
//                         break;
//                     }
//                 }
//                 Value::Soft(val) => {
//                     if val < 27 {
//                         let card = self.deck.draw().unwrap();
//                         println!("Dealer draws {}", &card);
//                         self.dealer.deal(card);
//                     } else {
//                         break;
//                     }
//                 }
//             }
//         }

//         println!(
//             "Dealer: {:?}, Player: {:?}",
//             self.dealer.score(),
//             self.player.score()
//         );
//         if self.dealer.score() == self.player.score() {
//             return State::Push(self.bet);
//         }
//         if (self.dealer.score().max() < self.player.score().min())
//             || (self.dealer.bust() && !self.player.bust())
//         {
//             return State::Win(self.bet);
//         }
//         State::Lose(self.bet)
//     }
// }
