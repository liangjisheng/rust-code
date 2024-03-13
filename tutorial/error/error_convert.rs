// 标准库、三方库、本地库，各有各的精彩，各也有各的错误。那么问题就来了，
// 我们该如何将其它的错误类型转换成自定义的错误类型？总不能神鬼牛魔，同台共舞吧
// 好在 Rust 为我们提供了 std::convert::From 特征

// pub trait From<T>: Sized {
//     fn from(_: T) -> Self;
// }

use std::fs::File;
use std::io;

#[derive(Debug)]
struct AppError {
    kind: String,    // 错误类型
    message: String, // 错误信息
}

// 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    // ? 可以将错误进行隐式的强制转换
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}

// --------------- 上述代码运行后输出 ---------------
// Error: AppError { kind: "io", message: "No such file or directory (os error 2)" }
