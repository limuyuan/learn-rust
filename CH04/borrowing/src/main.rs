fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let mut s3 = takes_and_gives_back(s2);

    let (mut s4, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s4, len);

    let mut p1 = &mut s3;
    //let mut p2 = &mut s3;

    change(&mut p1);
    //change(&mut p2);

    let s3_len = calculate_length_only(&s3);

    println!("The length of '{}' is {}.", s3, s3_len);

    println!("s4 = {}", s4);
    
    let r1 = &s4;
    let r2 = &s4;
    println!("r1 = {}, r2 = {}", r1, r2);

    let r3 = &mut s4;
    println!("r3 = {}", r3);
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

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
