// package: Cargo 提供的功能，一个包会含有一个 Cargo.toml 文件，是提供一系列功能的一个或多个 create
// create: 表示项目，是 Rust 中的独立编译单元。每个 create 对应生成一个库或可执行文件（.lib/.dll/.so/.exe）
// 模块（Modules）和 use： 允许你控制作用域和路径的私有性

// crate 是一个二进制项(binary)或者库(library)
// crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块(module)
// 包*（package) 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate

// 包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，
// 但是必须至少包含一个 crate（无论是库的还是二进制的）

// Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
// 如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同

// Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("add_to_wait_list");
        }

        pub fn add_to_wait_list_1() {
            println!("add_to_wait_list_1");
        }
    }
}

use crate::front_of_house::hosting;

// as 提供新的名称
use crate::front_of_house::hosting::add_to_wait_list as add;

// 使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的
// 如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，
// 那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：
// 我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域
pub use crate::front_of_house::hosting::add_to_wait_list_1;

// 层级共享
// use std::io;
// use std::io::Write;
// 可以改写为
// use std::io::{self, Write};

fn main() {
    //绝对路径访问
    crate::front_of_house::hosting::add_to_wait_list();

    // 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头
    self::front_of_house::hosting::add_to_wait_list();

    front_of_house::hosting::add_to_wait_list();

    hosting::add_to_wait_list();

    add();
}
