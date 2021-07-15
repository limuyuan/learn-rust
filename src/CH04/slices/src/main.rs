fn main() {
    println!("Hello, world!");

    let word = String::from("test case");

    let index = first_word_index(&word);
    
    let one = &word[0..3];
    let two = &word[3..];

    let chinese = String::from("测试");

    let cn1 = &chinese[0..6];

    println!("{}, {}", one, two);

    println!("{}", cn1);
    
    let first = first_word(&word);

    //word.clear();

    println!("The index of first space in word '{}' is {}.", word, index);

    println!("The first word is: {}", first);

    println!("The second word is: {}", second_word(&word));


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, [2, 3]);
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

fn first_word(s: &str) -> &str {
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
