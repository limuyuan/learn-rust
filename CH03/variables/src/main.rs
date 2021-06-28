use std::io;

fn main() {
    //let mut x = 5;
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    //x = 6;
    //println!("The value of x is: {}", x);
    let spaces = "   ";
    //let spaces = spaces.len();
    let spaces = spaces.len();
    println!("{}", spaces);

    //let c = 'z';
    //let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {}", heart_eyed_cat);
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("500: {}, 6.4: {}, 1: {}", five_hundred, six_point_four, one);

    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let b = [3; 5];
    let first = b[0];
    let second = b[1];

    println!("First = {}, second = {}.", first, second);

    println!("------Invalid Array Element Access Codes------");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a numer");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
