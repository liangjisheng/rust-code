// 标准库中的 std::io::Error 和 std::error::Error，前者是 IO 相关的错误结构体，
// 后者是一个最最通用的标准错误特征，同时前者实现了后者，因此 std::io::Error 可以
// 转换为 std:error::Error

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::{self, File};
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

// unwrap()：如果返回成功，就将 Ok(T) 中的值取出来，如果失败，就直接 panic
// let f = File::open("hello.txt").unwrap();
// expect 跟 unwrap 很像，也是遇到错误直接 panic, 但是会带上自定义的错误提示信息，相当于重载了错误打印的函数
// let f = File::open("hello.txt").expect("Failed to open hello.txt");

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

// ?运算符的作用：
// 如果Result是Ok：Ok中的值就是表达式的结果，然后继续执行程序
// 如果Result是Err：Err就是整个函数的返回值，就像使用了 return
// 其实 ? 就是一个宏，它的作用跟上面的 match 几乎一模一样

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

    fn read_username_from_file2() -> Result<String, io::Error> {
        // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
        fs::read_to_string("hello.txt")
    }
}

// ? 操作符会被传递给 from 函数，它定义于标准库的From trait中，用来将错误从一
// 种类型转换为另一种类型。
// ? 操作符收到的错误类型会被转换为当前函数要返回的错误类型
// 它可以自动进行类型提升（转换）
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

// 上面代码中 File::open 报错时返回的错误是 std::io::Error 类型，但是 open_file
// 函数返回的错误类型是 std::error::Error 的特征对象，可以看到一个错误类型通过 ? 返
// 回后，变成了另一个错误类型，这就是 ? 的神奇之处。

// 根本原因是在于标准库中定义的 From 特征，该特征有一个方法 from，用于把一个类型转成另外
// 一个类型，? 可以自动调用该方法，然后进行隐式类型转换。因此只要函数返回的错误 ReturnError
// 实现了 From<OtherError> 特征，那么 ? 就会自动把 OtherError 转换为 ReturnError。

// 这种转换非常好用，意味着你可以用一个大而全的 ReturnError 来覆盖所有错误类型，只需要为
// 各种子错误类型实现这种转换即可。

// ? 用于 Option 的返回

fn first(arr: &[i32]) -> Option<&i32> {
    let v = arr.get(0)?;
    Some(v)
}

//简化版本
fn first1(arr: &[i32]) -> Option<&i32> {
    arr.get(0)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 初学者在用 ? 时，老是会犯错，例如写出这样的代码
// fn first2(arr: &[i32]) -> Option<&i32> {
//     arr.get(0)?
// }

// 上面代码无法通过编译，切记：? 操作符需要一个变量来承载正确的值，这个函数只会返回
// Some(&i32) 或者 None，只有错误值能直接返回，正确的值不行，所以如果数组中存在
// 0 号元素，那么函数第二行使用 ? 后的返回类型为 &i32 而不是 Some(&i32)。因此
// ? 只能用于以下形式：
// let v = xxx()?;
// xxx()?.yyy()?;

// `?` operator can only be used in a function that returns `Result` or `Option`
// 主函数main()可以返回什么类型可以是下列两种
// ()：返回空
// Result<T, E>：返回有效值或者错误

// 实际上 Rust 还支持另外一种形式的 main 函数
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let f = File::open("hello.txt")?;
//
//     Ok(())
// }

// 这样就能使用 ? 提前返回了，同时我们又一次看到了Box<dyn Error> 特征对象，因为
// std::error:Error 是 Rust 中抽象层次最高的错误，其它标准库中的错误都实现了该
// 特征，因此我们可以用该特征对象代表一切错误，就算 main 函数中调用任何标准库函数
// 发生错误，都可以通过 Box<dyn Error> 这个特征对象进行返回。

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
