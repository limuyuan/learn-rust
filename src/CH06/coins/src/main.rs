enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Oklahoma,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("State quarter from {:?}!", UsState);
            25
        }
    }
}

fn main() {
    let quarter: Coin = Coin::Quarter(UsState::Alabama);
    println!("A Quarter is {} cents.", value_in_cents(quarter));

    // introduce `Option<T>`
    let some_number: Option<i32> = Some(5);
    let another_number: Option<i32> = Some(6);
    let some_string = Some("a String");
    let absent_number: Option<i32> = None;

    let number: i32 = 5;

    let sum = another_number / some_number;
}
