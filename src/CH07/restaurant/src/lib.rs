mod front_of_house;

use crate::front_of_house::hosting;
use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast, please", meal.toast);

    // seasonal_fruit is a private field
    // meal.seasonal_fruit = String::from("blueburries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod back_of_house;

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

use std::collections::HashMap;

fn test_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
