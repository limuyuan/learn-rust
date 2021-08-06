use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 30);

    // overwriting a value
    scores.insert(String::from("Yellow"), 35);

    // check before update
    println!("{:#?}", scores.entry(String::from("Blue")));
    println!("{:#?}", scores.entry(String::from("Orange")));
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Orange")).or_insert(60);

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

    let text = "hello hello world world wolrd test";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
