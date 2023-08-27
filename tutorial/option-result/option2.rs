#[allow(unused)]
fn match_ref_some() {
    let some = Some(String::from("hello"));
    let ref_some = &some;
    match ref_some {
        Some(s) => println!("{}", s),
        None => println!("no string"),
    }

    match some {
        Some(s) => println!("{}", s),
        None => println!("no string"),
    }

    // println!("{}", some.unwrap()); //error
}

fn find_store(mobile_os: &str) -> Option<&str> {
    match mobile_os {
        "iOS" => Some("App Store"),
        "android" => Some("Play Store"),
        _ => None,
    }
}

fn match1() {
    // Option 和 Result 这两个枚举都拥有范型，由 Rust 标准库提供
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // 读取一个环境变量，成功时触发 Ok 函数，失败时触发 Err 函数
    match std::env::var("HOME") {
        Ok(data) => println!("Data: {}", data),
        Err(err) => println!("Error: {}", err),
    }

    match std::env::var("xxx") {
        Ok(data) => println!("Data: {}", data),
        Err(err) => println!("Error: {}", err), // Error: environment variable not found
    }
}

fn main() {
    // match_ref_some();

    println!(
        "{}",
        match find_store("windows") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );

    match1();
}
