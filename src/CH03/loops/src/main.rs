fn main() {
    /*loop {
        println!("again!");
    }*/

    let mut counter = 0;

    /*let result = loop {
        counter += 1;

        println!("counter = {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    
    };*/

    while counter < 10 {
        println!("counter = {}", counter);

        counter += 1;
    }

    let result = counter * 2;

    println!("The result is {}\n", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
    
        index += 1;

        println!("the index is: {}", index);
    }

    println!("---For loop---");

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..10).rev() {
        println!("counter = {}", number);
    }
}
