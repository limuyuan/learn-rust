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

    let test = plus_one_int(some_number);
    let test2 = plus_one(some_number);

    
    //let sum = another_number / some_number;
    println!("5+1={}", plus_one_int(some_number));
    println!("None=0, so None+1={}", plus_one_int(absent_number));

    println!("{:?}", plus_one(some_number));

    let a_string: Option<String> = Some(String::from("Test"));
    println!("{:?}", return_string(a_string));
    //String have ownership
    //println!("{:?}", return_string(a_string));


}

fn plus_one_int(i: Option<i32>) -> i32 {
    match i {
        Some(i) => i + 1,
        None => 0 + 1,
    }
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn return_string(s: Option<String>) -> Option<String> {
    match s {
        None => None,
        Some(s) => Some(s),
    }
}
