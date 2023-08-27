// pub use a::b;	Bring a::b into scope and reexport from here.

pub mod a {
    pub mod b {
        pub mod c {
            pub fn nested_function() {
                println!("Nested function");
            }

            pub fn nested_function_1() {
                println!("Nested function 1");
            }
        }
    }
}

use a::b::c::nested_function_1;

// 枚举也是类似模块之类的命名空间的一种形式。 因此，可以使用use关键字将枚举变体带入范围中
#[derive(Debug)]
enum FlagColor {
    FlagOrange,
    FlagWhite,
    FlagGreen,
}

use FlagColor::{FlagGreen, FlagOrange, FlagWhite};

// *运算符用于将所有项目放入范围，这也称为glob运算符。 如果使用glob运算符，那么不需要单独指定枚举变量
#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Green,
    Orange,
}

use Color::*;

// super关键字用于从当前模块访问父模块，它使能够访问父模块的私有功能
mod father {
    fn x() -> u8 {
        5
    }

    pub mod example {
        use super::x;

        pub fn foo() {
            println!("{}", x());
        }
    }
}

fn main() {
    a::b::c::nested_function();
    nested_function_1();

    let _o = FlagOrange;
    let _w = FlagWhite;
    let _g = FlagGreen;
    println!("{:?}", _o);
    println!("{:?}", _w);
    println!("{:?}", _g);

    let _red = Red;
    let _yellow = Yellow;
    let _green = Green;
    let _orange = Orange;
    println!("{:?}", _red);
    println!("{:?}", _yellow);
    println!("{:?}", _green);
    println!("{:?}", _orange);

    father::example::foo();
}
