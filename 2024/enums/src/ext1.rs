fn afunction() {
    let anumber = Some(23);
    let afloat = Some(34.234);
    let astring = Some("asd");
    let none: Option<u32> = None;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    SomeState1,
    SomeState2,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("this quarter comes from {#:?}", state);
            return 25;
        },
    }
}

fn plus_one(some: Option<u32>) -> Option<u32> {
    match some {
        None => some,
        Some(i) => {
            return Some(i + 1);
        }
    }
}

fn plus_two(v: Option<u32>) -> Option<u32> {
    match v {
        Some(i) => {
            return Some(i+2);
        },
        _ => (), 
    }
}

fn some2() {
    if let Some(3) = x {
        println!("{}",Some(3));
    }
}