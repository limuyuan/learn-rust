use std::{collections::HashMap, io};

fn main() {
    let mut company: HashMap<String, String> = HashMap::new();

    loop {
        let mut input = String::new();

        println!("Add+[Name]+to+[Dept] or End|Query:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input_vec: Vec<&str> = input.trim().split(" ").collect();

        if input_vec[0] == "Add" && input_vec.len() == 4 && input_vec[2] == "to" {
            company.insert(String::from(input_vec[1]), String::from(input_vec[3]));
        } else if input_vec[0] == "End" {
            break;
        } else if input_vec[0] == "Query" {
            println!();
            println!("People in company:");
            for (name, dept) in &company {
                println!("{} from {}", name, dept);
            }
        } else {
            println!("Invalid input: {}", input.trim());
        }
    }
}
