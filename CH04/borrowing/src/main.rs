fn main() {
      let s1 = gives_ownership();

      let s2 = String::from("hello");

      let s3 = takes_and_gives_back(s2);

      let (s4, len) = calculate_length(s1);

      println!("The length of '{}' is {}.", s4, len);

      let s3_len = calculate_length_only(&s3);

      println!("The length of '{}' is {}.", s3, s3_len);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_only(s: &String) -> usize {
    let length = s.len();
    length
}
