#[derive(Debug)]
enum Language {
    Go,
    Rust,
    JavaScript,
}

#[derive(Debug)]
enum OpenJS {
    Nodejs,
    React,
}

#[derive(Debug)]
enum Language1 {
    JavaScript(OpenJS),
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn e1() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?} \n {:#?} \n", home, loopback);

    let lan = Language::Go;
    println!("{:?}", lan);
    let open = OpenJS::Nodejs;
    println!("{:?}", open);
}

// Rust 中的枚举值可以有它们自己的数据类型

#[derive(Debug)]
enum GenderCategory {
    Name(String),
    Usr_ID(i32),
}

fn e2() {
    let p1 = GenderCategory::Name(String::from("alice"));
    let p2 = GenderCategory::Usr_ID(100);
    println!("{:?}", p1);
    println!("{:?}", p2);

    match p1 {
        GenderCategory::Name(val) => {
            println!("{}", val);
        }
        GenderCategory::Usr_ID(val) => {
            println!("{}", val);
        }
    }
}

fn main() {
    // e1();
    e2();
}
