fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Morris"),
        active: true,
        sign_in_count: 1,
    };
   
    user1.email = String::from("another@example.com");


    println!("{}", user1.email);


    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));

    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@3.com"), 
        ..user2
    };

    println!("user3's username = {}", user3.username);
    println!("email of user2: {}, user3: {}", user2.email, user3.email);

    let color = Color(255, 255, 255);

    println!("{}", color.0); 
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
