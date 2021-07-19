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
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) { }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum { 
    V4(u8, u8, u8, u8),
    V6(String),
}
