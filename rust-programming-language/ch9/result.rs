use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn r1() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn r2() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn r3() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// unwrap
// 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
// 如果 Result 是成员 Err，unwrap 会为我们调用 panic!

// expect 允许我们选择 panic! 的错误信息
// expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，
// 而不像 unwrap 那样使用默认的 panic! 信息

fn r4() {
    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? 定义为与处理 Result 值的 match 表达式有着完全相同的工作方式
// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
// 如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，
// 这样错误值就被传播给了调用者

// match 表达式与 ? 运算符所做的有一点不同：? 运算符所使用的错误值被传递给了
// from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为
// 另一种类型。当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数
// 返回类型所指定的错误类型。这在当函数返回单个错误类型来代表所有可能失败的方式
// 时很有用，即使其可能会因很多种原因失败

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// ? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接
// 使用链式方法调用来进一步缩短代码

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数
fn r5() {
    // 编译失败, 函数返回 (), 而 ? 返回 Result
    // let greeting_file = File::open("hello.txt")?;
}

// ? 也可用于 Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // r1();
    // r2();
    r3();
}

// main 函数也可以返回 Result<(), E>
// Box<dyn Error> 类型是一个 trait 对象（trait object）
fn main1() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
