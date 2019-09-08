use blackjack;
use blackjack::*;

// fn logic<'a>(view: View<'a>) -> Action {
//     if view.dealer.score().max() < 7 && view.player.score().min() >= 13 {
//         dbg!(Action::Stand)
//     } else {
//         if view.player.score().min() >= 17 {
//             dbg!(Action::Stand)
//         } else {
//             dbg!(Action::Hit)
//         }
//     }
// }

// fn basic_strategy<'a>(view: View<'a>) -> Action {
//     let d = view.dealer.score().max();
//     println!(
//         "Current: {:?} {:?} vs {:?}",
//         view.player.cards,
//         view.player.score(),
//         view.dealer
//     );
//     let a = match view.player.score() {
//         Value::Soft(val) => match val {
//             0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => Action::Hit,
//             8 => {
//                 if d == 5 || d == 6 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             9 => {
//                 if d < 7 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             10 => {
//                 if d < 10 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             11 => Action::Double,
//             12 => {
//                 if d == 4 || d == 5 || d == 6 {
//                     Action::Stand
//                 } else {
//                     Action::Hit
//                 }
//             }
//             13 | 14 | 15 | 16 => {
//                 if d < 7 {
//                     Action::Stand
//                 } else {
//                     Action::Hit
//                 }
//             }
//             _ => Action::Stand,
//         },
//         Value::Hard(val) => match val {
//             0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => Action::Hit,
//             8 => {
//                 if d == 5 || d == 6 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             9 => {
//                 if d < 7 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             10 => {
//                 if d < 10 {
//                     Action::Double
//                 } else {
//                     Action::Hit
//                 }
//             }
//             11 => Action::Double,
//             12 => {
//                 if d == 4 || d == 5 || d == 6 {
//                     Action::Stand
//                 } else {
//                     Action::Hit
//                 }
//             }
//             13 | 14 | 15 | 16 => {
//                 if d < 7 {
//                     Action::Stand
//                 } else {
//                     Action::Hit
//                 }
//             }
//             _ => Action::Stand,
//         },
//     };
//     println!("{:?}", a);
//     a
// }

fn main() {
    let player = Player::new(10_000);

    let mut game = Game::init(player);
    let mut view = game.bet(100).unwrap();

    while view.state == blackjack::game::State::Player {
        println!(
            "P: {} ({})\tD: {} ({})",
            view.player.hand,
            view.player.score(),
            view.dealer,
            view.dealer.score()
        );
        let action = if view.player.score() < 17 && view.dealer.score() > 6 {
            Action::Hit
        } else {
            Action::Stand
        };
        view = match game.action(action) {
            Ok(view) => view,
            Err(e) => panic!("Error encountered!: {:?}", e),
        }
    }

    while view.state == blackjack::game::State::Dealer {
        println!(
            "P: {} ({})\tD: {} ({})",
            view.player.hand,
            view.player.score(),
            view.dealer,
            view.dealer.score()
        );
        view = game.update().unwrap();
    }

    println!("{:#?}", view);

    println!("{:#?}", game.finish());

    // let res = game.play(10, basic_strategy);

    // println!("{:?}", res);
}
