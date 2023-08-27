// 运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作
// Rust 并不允许创建自定义运算符或重载任意运算符，不过 std::ops 中所列出的运算符和相应的
// trait 可以通过实现运算符相关 trait 来重载

// trait Add<RHS=Self> {
//     type Output;
//
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// 这里的RHS=Self是默认类型参数（default type parameters）。RHS 是一个泛型类型参数
// （“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数
// 如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 自定义类型
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
