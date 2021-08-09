fn main() {
    let origin_word = String::from("school");
    println!("{}", pig_latin(&origin_word));
}

fn pig_latin(s: &str) -> String {
    let mut return_str = String::from(s);
    match &s[0..1] {
        "a"|"e"|"i"|"o"|"u" => {
            return_str.push_str("-hay");
        }
        first_letter => {
            return_str.remove(0);
            return_str.push('-');
            return_str.push_str(first_letter);
            return_str.push_str("ay");
        }
    }
    String::from(return_str)
}
