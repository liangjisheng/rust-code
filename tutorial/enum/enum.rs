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

// 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn e3() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
    // heart 已经被消耗掉了，下面的代码会报错
    // print_suit(heart);
}

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}", card);
}

// 给每张牌赋予一个值

#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn e4() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);

    println!("{:?}", c1);
    println!("{:?}", c2);
}

// 同一个枚举类型下的不同成员还能持有不同的数据类型

#[derive(Debug)]
enum PokerCard1 {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn e5() {
    let c1 = PokerCard1::Spades(5);
    let c2 = PokerCard1::Diamonds('A');

    println!("{:?}", c1);
    println!("{:?}", c2);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 枚举类型之所以强大，不仅仅在于它好用、可以同一化类型，还在于，我们可以像结构体一样，为枚举实现方法：

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        println!("call");
    }
}

fn e6() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);

    m1.call();
}

fn main() {
    // e1();
    // e2();
    // e3();
    // e4();
    // e5();
    e6();
}
