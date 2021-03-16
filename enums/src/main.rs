//use crate::Coin::Penny;

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    }


fn main() {
    //let home = IpAddr::V4(127, 0, 0, 1);
    //let loopback = IpAddr::V6(String::from("::1"));
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of six is {:?} while the value of none is {:?}",six,none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
    None => None,
    Some(i) => Some(i + 1),
    }
    }


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    }

fn value_in_cents(coin: Coin) -> u8 {
match coin {
Coin::Penny => {
println!("Lucky penny!");
1
}
Coin::Nickel => 5,
Coin::Dime => 10,
Coin::Quarter(state) => {
    println!("State quarter from {:?}!", state);
    25
    }
}
}
