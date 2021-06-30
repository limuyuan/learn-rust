fn main() {
    println!("Hello, world!");

    let mut word = String::from("one two");

    let index = first_word(&word);
    
    let one = &word[0..3];
    let two = &word[4..];

    let chinese = String::from("测试");

    let cn1 = &chinese[0..2];

    println!("{}, {}", one, two);

    println!("{}", cn1);
    
    word.clear();

    println!("The index of first space in word '{}' is {}.", word, index);
}

fn first_word(s: &String) -> usize {
    // type of bytes is &[u8]
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
