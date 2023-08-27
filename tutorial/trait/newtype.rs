// newtype模式(newtype pattern)的作用是绕开孤儿规则(只要 trait 或类型
// 对于当前 crate 是本地的话就可以在此类型上实现该 trait)

// newtype 模式涉及到在一个元组结构体中创建一个新类型。这个元组结构体带有一个
// 字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的
// 这样就可以在这个封装上实现 trait。使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。

// 示例：如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，因为
// Display trait 和 Vec<T> 都定义于我们的 crate 之外。可以创建一个包含 Vec<T>
// 实例的 Wrapper 结构体，接着在 Wrapper 上实现 Display 并使用 Vec<T> 的值

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
