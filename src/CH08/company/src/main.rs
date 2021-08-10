use std::{io, collections::HashMap};

fn main() {
    println!("Hello, world!");
    
    let company: HashMap<String, String> = HashMap::new();
    
    
    loop {
        let mut input = String::new();

        println!("Add+[Name]+to+[Dept] or End:");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input_vec: Vec<&str> = input.trim().split(" ").collect();

        for s in &input_vec {
            //println!("{}", s);
        }

        // println!("input_vec[0]{}", input_vec[0]);

        if input_vec[0] == "Add" {
        } else if input_vec[0] == "End" {
            break;
        } else {
            println!("Invalid input: {}", input_vec[0]);
        }
    }
}

fn add_to_dept(name: String) {}
