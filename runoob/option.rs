// Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 初始值为空的 Option 必须明确类型
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {}
    }

    // if let 语法
    let i1 = 0;
    if let 0 = i1 {
        println!("zero");
    }

    enum Book {
        Papery(u32),
        Electronic(String),
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
