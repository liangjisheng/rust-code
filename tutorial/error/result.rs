// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// unwrap()：如果没有返回错误，则继续执行，否则将错误以panic的形式抛出。
// expect("自定义错误信息")：如果没有返回错误，则继续执行，否则将自定义错误信息以panic的形式抛出

// ? 操作符会被传递给 from 函数，它定义于标准库的From trait中，用来将错误从一种类型转换为另一种类型。
// ? 操作符收到的错误类型会被转换为当前函数要返回的错误类型

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn r1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // 可恢复错误处理，可根据需要返回一个值或者直接panic!
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

fn r2() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 由于在rust中区分了可恢复错误和不可恢复错误，不可恢复错误会使用panic!抛出，
// 而可恢复错误则通过返回Result的形式给出。因此，这里所说的传播错误，主要讨论
// 的是可恢复错误的传播，想要把错误传播到上层，通常是在match中匹配到错误以后，
// 重新封装到一个新的Err中返回
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 传播错误1：把打开文件的错误传递给调用者
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 传播错误2：把读取文件的错误传递给调用者
    }
}

fn r3() {
    let res = read_username_from_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(err) => println!("{}", err),
    }
}

// ?运算符只能应用在Result类型的函数上
// ?运算符的作用：
// 如果Result是Ok：Ok中的值就是表达式的结果，然后继续执行程序
// 如果Result是Err：Err就是整个函数的返回值，就像使用了 return

fn r4() {
    fn read_username_from_file() -> Result<String, io::Error> {
        // 相当于还告诉调用方有可能发生错误
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // 改成链式调用
    fn read_username_from_file1() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}

// 主函数main()可以返回什么类型可以是下列两种
// ()：返回空
// Result<T, E>：返回有效值或者错误

// Trait std::convert::From上有一个from函数：用于错误之间的转换。
// 被?所应用的错误，会隐式的被from函数处理。from接收的错误类型会被转化为当前函数返回类型所定义的错误类型。
// 用途：针对不同错误原因，返回同一种错误类型。
// 前提条件：每个错误类型实现了转换为所返回的错误类型的from函数

fn main() {
    // r1();
    // r2();
    // r3();
    r4();
}
