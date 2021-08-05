use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 30);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![20, 10];

    let scores1: HashMap<_, _> = teams.into_iter().zip(initial_score.into_iter()).collect();

    println!("{:#?}", scores1);
    
    let field_name = String::from("Red");
    let field_value = 0;

    let mut map = HashMap::new();

    map.insert(&field_name, field_value);

    println!("{}, {}, {:?}", field_name, field_value, map);

    println!("{:?}, {:?}", map.get(&field_name), map.get(&String::from("test")));
    
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}
