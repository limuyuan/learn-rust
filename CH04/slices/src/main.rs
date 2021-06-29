fn main() {
    println!("Hello, world!");

    let word = String::from("one two");
    println!("The index of first space in word '{}' is {}.", word, first_word(&word));
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
