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
}
