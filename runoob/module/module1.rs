// 每一个 Rust 文件的内容都是一个"难以发现"的模块
mod second_module;

mod nation {
    pub mod government {
        pub fn govern() {
            println!("government")
        }
        pub fn demo() {
            println!("demo")
        }
    }
    pub fn govern() {
        println!("nation")
    }

    // use 关键字可以与 pub 关键字配合使用
    // pub use government::demo;
}
// use 关键字能够将模块标识符引入当前作用域 解决了局部模块路径过长的问题
use crate::nation::government::govern;
// 使用 as 关键字为标识符添加别名
use crate::nation::govern as nation_govern;

use std::f64::consts::PI;

fn main() {
    println!("This is the main module.");
    println!("{}", second_module::message());

    govern();
    nation_govern();

    println!("{}", (PI / 2.0).sin());
}
