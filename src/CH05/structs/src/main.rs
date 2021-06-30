fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Morris"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("{}", user1.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
