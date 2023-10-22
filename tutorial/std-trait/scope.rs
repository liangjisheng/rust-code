// 只有当 trait 在作用域之中时，trait 项才能被使用。大多数 Rustaceans
// 在第一次尝试写一个 I/O 相关的程序时，都会在吃过一番苦头之后了解到这一点，
// 因为Read和Write的 trait 并不在标准库的预置（prelude）中

use std::fs::File;
use std::io;

// read_to_string(buf: &mut String) 声明于 std::io::Read 中并且被
// std::fs::File 结构体实现，但是要想调用它，std::io::Read 必须在当前作用域中
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(())
}

// 标准库预置（The standard library prelude）是标准库中的一个模块，也就是说
// std::prelude::v1，它在每个其他模块的顶部被自动导入，即use std::prelude::v1::*
// 这样的话，下面这些 trait 就总会在作用域中，我们不需要自己显式地导入它们，因为它们是预置的一部分
