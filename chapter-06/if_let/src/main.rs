#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}


// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
// }


// fn main() {
//     let some_u8_value = Some(3u8);
//     if let Some(3) = some_u8_value {
//         println!("three");
//     }
// }


// fn main() {
//     let some_u8_value = Some(3u8);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }
// }
