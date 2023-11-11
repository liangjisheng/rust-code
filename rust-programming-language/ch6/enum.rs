// 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。也就是说，
// IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用。
// 作为定义枚举的结果，这些构造函数会自动被定义

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法
impl Message {
    fn call(&self) {
        println!("message call")
    }
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m1 = Message::Quit;
    m1.call();
    let m2 = Message::Write(String::from("write"));
    m2.call();
}
