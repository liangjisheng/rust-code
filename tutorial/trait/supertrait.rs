// 为了实现在Trait1实现中调用Trait2的实现，需要规定这两个trait的关系
// 可以将 Trait2 声明为 Trait1 的 super trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // 调用了Display的行为
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

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 0, y: 1 };
    p.outline_print();
}
