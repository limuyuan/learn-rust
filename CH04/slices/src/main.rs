#[allow(unused_mut)]
fn main() {
    println!("Hello, world!");

    let mut word = String::from("test case");

    let index = first_word_index(&word);
    
    let one = &word[0..3];
    let two = &word[3..];

    let chinese = String::from("测试");

    let cn1 = &chinese[0..6];

    println!("{}, {}", one, two);

    println!("{}", cn1);
    
    //word.clear();

    println!("The index of first space in word '{}' is {}.", word, index);

    println!("The first word is: {}", first_word(&word));

    println!("The second word is: {}", second_word(&word));
}

fn first_word_index(s: &String) -> usize {
    // type of bytes is &[u8]
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }
    return "only 1 word!";
}
