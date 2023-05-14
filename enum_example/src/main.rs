#[derive(Debug)] // 使我们能够打印并观察各州的设计
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Option 有两个值，要么有值，要么无值
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let a = value_in_cents(Coin::Quarter(UsState::Alaska));

    
    let five = Some(5); // 可以对some进行操作
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:#?}, none is {:#?}", six.unwrap(), none);

    println!("Hello, world! {:?}", a);
}
