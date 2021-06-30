fn main() {
    println!("Hello, world!");

    let mut word = String::from("one two");

    let index = first_word(&word);
    
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
