// 使用 trait bound 有条件地实现方法
// 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。对任何满足特定 trait bound
// 的类型实现 trait 被称为 blanket implementations，它们被广泛的用于 Rust 标准库中。
// 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait

// impl<T: Display> ToString for T {
// --snip--
// }

// 因为标准库有了这些 blanket implementation，我们可以对任何实现了 Display trait 的
// 类型调用由 ToString 定义的 to_string 方法。例如，可以将整型转换为对应的 String 值，
// 因为整型实现了 Display

// let s = 3.to_string();
