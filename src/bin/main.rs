use blackjack;
use blackjack::*;
use rand::prelude::*;

fn basic_strategy(view: &View, idx: usize) -> Action {
    let d = view.dealer.score();
    if view.player.can_split(idx) {
        match view.player.hands[idx].score() {
            10 | 12 => Action::Hit,
            20 => Action::Stand,
            _ => Action::Split,
        }
    } else if view.player.hands[idx].soft() {
        match view.player.hands[idx].score() {
            13 | 14 | 15 | 16 => {
                if d >= 4 && d <= 6 {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            17 => {
                if d < 7 {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            18 => {
                if d >= 3 && d <= 6 {
                    Action::Double
                } else if d >= 9 && d <= 10 {
                    Action::Hit
                } else {
                    Action::Stand
                }
            }
            19 => {
                if d == 6 {
                    Action::Double
                } else {
                    Action::Stand
                }
            }
            _ => Action::Stand,
        }
    } else {
        match view.player.hands[idx].score() {
            2 | 3 | 4 | 5 | 6 | 7 => Action::Hit,
            8 => {
                if d == 5 || d == 6 {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            9 => {
                if d <= 6 {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            10 => {
                if d <= 9 {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            11 => Action::Double,
            12 => {
                if d >= 4 && d <= 6 {
                    Action::Stand
                } else {
                    Action::Hit
                }
            }
            13 | 14 | 15 | 16 => {
                if d < 7 {
                    Action::Stand
                } else {
                    Action::Hit
                }
            }
            _ => Action::Stand,
        }
    }
}

fn display_view(view: &View) {
    println!(
        "P: {}\tD: {} ({})",
        view.player
            .hands
            .iter()
            .map(|h| format!("[{}]({})", h, h.score()))
            .collect::<Vec<_>>()
            .join(", "),
        view.dealer,
        view.dealer.score()
    );
}

fn simulate(
    rules: Ruleset,
    bankroll: usize,
    bet: usize,
    occurrences: usize,
    display: bool,
) -> String {
    let mut player = Player::new(bankroll);
    let mut wins = 0;
    let mut bj = 0;
    let mut total = 0;

    let mut server_rng =
        blackjack::pcg::PCG32::new(thread_rng().next_u64(), thread_rng().next_u64());
    let mut server_seed = server_rng.to_seed();

    let mut player_rng = blackjack::pcg::PCG32::new(44, 54);
    let mut player_seed = player_rng.to_seed();

    for _ in 0..occurrences {
        let mut game = Game::init(rules, player, &mut server_rng);

        if display {
            dbg!(game.sha256());
        }
        game.player_shuffle(&mut player_rng);
        if display {
            dbg!(game.sha256());
        }

        let mut view = game.bet(bet).unwrap();

        while let State::Player(idx) = view.state {
            if display {
                display_view(&view);
            }
            let action = basic_strategy(&view, idx);
            view = match game.action(action) {
                Ok(view) => view,
                Err(e) => panic!(
                    "Player error encountered!: {:?} {:?} {} {:#?}",
                    e, action, idx, view
                ),
            };
        }

        while view.state == blackjack::game::State::Dealer {
            if display {
                display_view(&view);
            }
            view = match game.dealer() {
                Ok(view) => view,
                Err(e) => panic!("Dealer error encountered!: {:?} {:#?}", e, view),
            };
        }

        total += view.scores.len();
        for score in view.scores {
            match score {
                Outcome::Win(_) => wins += 1,
                Outcome::Blackjack(_) => bj += 1,
                _ => {}
            }
        }
        player = game.finish().unwrap();

        // Now make a mock deck to check everything was fair
        let mut deck = Deck::new(rules.decks);
        server_rng = blackjack::pcg::PCG32::from_seed(server_seed);
        player_rng = blackjack::pcg::PCG32::from_seed(player_seed);
        deck.shuffle(&mut server_rng);
        dbg!(deck.notation());
        dbg!(deck.sha256());
        deck.shuffle(&mut player_rng);
        dbg!(deck.notation());
        dbg!(deck.sha256());
    }

    format!(
        "wins {:8}\tbj {:8}\ttotal {:8}\tcash {:8}\tP/L per wager {}",
        wins,
        bj,
        total,
        player.chips,
        (player.chips as f64 - bankroll as f64) / total as f64,
    )
}

fn main() {
    let rules = Ruleset::default().decks(6);

    // println!("{}", simulate(10000, 10, 100));
    println!("{}", simulate(rules, 1_000_000, 1, 1, true));
    // println!(
    //     "{}",
    //     simulate(rules.decks(1).stand(false), 1_000_000, 1, 50_000)
    // );
    // println!(
    //     "{}",
    //     simulate(rules.decks(1).stand(true), 1_000_000, 1, 50_000)
    // );

    // let mut thread_rng = thread_rng();

    // let mut deck = blackjack::Deck::new(1);
    // println!("{}", deck.notation());
    // deck.shuffle(&mut rng);
    // println!("{}", deck.notation());

    // let mut rng = blackjack::pcg::PCG32::new(42, 54);
    // dbg!(rng.next_u64());
    // dbg!(rng.next_u64());
    // dbg!(rng.to_seed().sha256());

    // let mut rng2 = blackjack::pcg::PCG32::from_seed(rng.to_seed());
    // dbg!(rng2.to_seed().sha256());

    // dbg!(rng.next_u64());
    // dbg!(rng2.next_u64());
}
