fn main() {
    let four = IpAddKind::ipv4;
    let six = IpAddKind::ipv6;
    let home = IpAdd {
        kind: IpAddKind::ipv4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAdd {
        kind: IpAddKind::ipv6,
        address: String::from("::1")
    };
    let home = IpAddEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddEnum::V6(String::from("::1"));  

    let home = IpAdd2::V4(127, 0, 0, 1);
    let loopback = IpAdd2::V6(String::from("::1"));
}

enum IpAdd2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum IpAddEnum {
    V4(String),
    V6(String)
}

struct IpAdd {
    kind: IpAddKind,
    address: String
}

#[derive(Debug)]
enum IpAddKind {
    ipv4,
    ipv6
}

fn route(ipv: IpAddKind) {
    println!("{:#?}",ipv);
}
