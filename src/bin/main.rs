use blackjack;
use blackjack::*;

fn basic_strategy(view: &View) -> Action {
    let d = view.dealer.score();
    if view.player.is_splittable() {
        Action::Split
    } else if view.player.soft() {
        match view.player.score() {
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
        match view.player.score() {
            5 | 6 | 7 => Action::Hit,
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
        "P: {} ({})\tD: {} ({})",
        view.player.hands[view.player.active],
        view.player.score(),
        view.dealer,
        view.dealer.score()
    );
}

fn main() {
    let mut player = Player::new(100_000);

    let mut game = Game::init(player);
    let mut view = game.bet(1).unwrap();
    while view.states[view.active] == State::Player {
        display_view(&view);
        let action = basic_strategy(&view);
        view = match game.action(action) {
            Ok(view) => view,
            Err(e) => panic!("Error encountered!: {:?}", e),
        }
    }

    while view.states[view.active] == blackjack::game::State::Dealer {
        display_view(&view);
        view = game.update().unwrap();
    }
    dbg!(&view);
    player = game.finish().unwrap();
    println!("{}", player.chips);
}

// fn main() {
//     let mut player = Player::new(100_000);

//     let mut wins = 0;
//     let mut bj = 0;
//     let mut total = 0;

//     for i in 0..100_000 {
//         let mut game = Game::init(player);
//         let mut view = game.bet(1).unwrap();

//         while view.state == blackjack::game::State::Player {
//             // display_view(&view);
//             // let action = if view.player.score() < 17 && view.dealer.score() > 6 {
//             //     Action::Hit
//             // } else {
//             //     Action::Stand
//             // };

//             let action = basic_strategy(&view);

//             view = match game.action(action) {
//                 Ok(view) => view,
//                 Err(e) => panic!("Error encountered!: {:?}", e),
//             }
//         }
//         // display_view(&view);

//         while view.state == blackjack::game::State::Dealer {
//             // display_view(&view);
//             view = game.update().unwrap();
//         }

//         match view.state {
//             State::Final(fin) => match fin {
//                 Outcome::Win(_) => wins += 1,
//                 Outcome::Blackjack(_) => bj += 1,
//                 _ => {}
//             },
//             _ => panic!("{:?}", view.state),
//         }
//         player = game.finish().unwrap();
//         total += 1;
//     }

//     println!(
//         "wins {} bj {} total {} cash {}",
//         wins, bj, total, player.chips
//     );
//     println!(
//         "WR: {}%, BJ: {}%",
//         100.0 * wins as f32 / total as f32,
//         100.0 * bj as f32 / total as f32
//     );

//     // let res = game.play(10, basic_strategy);

//     // println!("{:?}", res);
// }
