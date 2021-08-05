use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 30);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![20, 10];

    let scores1: HashMap<_, _> = teams.into_iter().zip(initial_score.into_iter()).collect();

    println!("{:#?}", scores1);
}
