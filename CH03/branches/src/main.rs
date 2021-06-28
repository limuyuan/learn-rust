fn main() {
    let number = 6;

    if number > 7 {
        println!("condition was true");
    } else if number == 7 {
        println!("condition was false");
    } else {
        println!("third arm");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {}", number);
}
