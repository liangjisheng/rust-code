// 为了启用多所有权需要显式地使用 Rust 类型 Rc<T>
// 其为 引用计数（reference counting）的缩写
// 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用
// 如果某个值有零个引用，就代表没有任何有效引用并可以被清理

// 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据。如果 Rc<T> 也允许多个可变引用
// 则会违反借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// 修改 List 的定义为使用 Rc<T> 代替 Box<T>
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // 展示不能用两个 Box<T> 的列表尝试共享第三个列表的所有权
    // value used here after move
    // let c = Cons(4, Box::new(a));

    // 当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>
    // 这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权
    // 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。每次调用 Rc::clone
    // Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 打印引用计数
    println!("count after creating a = {}", Rc::strong_count(&a));

    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        // 当 Rc<T> 值离开作用域时自动减少引用计数
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
