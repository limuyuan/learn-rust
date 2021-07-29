fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home1 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback1 = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("test"));
    m.call();

    let n = Message::ChangeColor(255, 255, 255);
    n.call();

    let mov = Message::Move { x: 1, y: 2 };
    mov.call();

    // introduce `Option<T>`
    let some_number: Option<i32> = Some(5);
    let another_number: Option<i32> = Some(6);
    let some_string = Some("a String");
    let absent_number: Option<i32> = None;

    let number: i32 = 5;

    let sum = another_number / some_number;
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
