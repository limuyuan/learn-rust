fn main() {
    println!("Hello, world!");

    let x = 5;

    let _y = {
        let x = 3;
        x + 1
    };

    let z = plus_one(five());

    another_function(x, z);
    
    //let x = (let y = 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
