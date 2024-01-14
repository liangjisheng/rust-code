// 为了实现在 Trait1 实现中调用 Trait2 的实现，需要规定这两个 trait 的关系
// 可以将 Trait2 声明为 Trait1 的 super trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // 调用了 Display 的行为
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string，
// 其会为任何实现 Display 的类型自动实现。如果不在 trait 名后增加 : Display 并尝试在
// outline_print 中使用 to_string，则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 0, y: 1 };
    p.outline_print();
}
