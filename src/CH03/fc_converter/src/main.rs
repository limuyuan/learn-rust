use std::io;

fn main() {
    let mut number = String::new();

    //println!("{}", cf_selector());
    if cf_selector() == "F" {
        println!("Please input Fahrenheit:");
        io::stdin().read_line(&mut number).expect("Failed to read!");

        loop {
            match number.trim().parse() {
                Ok(num) => {
                    println!("Celcius = {:.2}", f_to_c(num));
                    break;
                }
                Err(_) => {
                    println!("Please input a valid number!");
                    break;
                }
            };
        }
    } else {
        println!("Please input Celsius:");
        io::stdin().read_line(&mut number).expect("Failed to read!");

        loop {
            match number.trim().parse() {
                Ok(num) => {
                    println!("Fahrenheit = {:.2}", c_to_f(num));
                    break;
                }
                Err(_) => {
                    println!("Please input a valid number!");
                    break;
                }
            };
        }
    }
}

fn cf_selector() -> String {
    loop {
        let mut cf: String = String::new();

        println!("Please input C for Celsius and F for Fahrenheit:");
        println!("Then I will convert your input to another unit.");

        io::stdin().read_line(&mut cf).expect("Fail to read");

        let cf = cf.trim();

        if cf == "C" || cf == "c" {
            println!("You have chosen Celsius!");
            break String::from("C");
        } else if cf == "F" || cf == "f" {
            println!("You have chosen Fahrenheit!");
            break String::from("F");
        } else {
            println!("Please input a valid letter!");
        }
    }
}

fn c_to_f(cel: f64) -> f64 {
    32.0 + cel * 1.8
}

fn f_to_c(fahr: f64) -> f64 {
    (fahr - 32.0) * (5.0 / 9.0)
}
