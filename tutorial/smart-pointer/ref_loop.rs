// Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak））
// 但并不是不可能。与在编译时拒绝数据竞争不同，Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在
// Rust 被认为是内存安全的。这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的
// 这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // a

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

// 可以看到将列表 a 修改为指向 b 之后， a 和 b 中的 Rc<List> 实例的引用计数都是 2。
// 在 main 的结尾，Rust 丢弃 b，这会使 b Rc<List> 实例的引用计数从 2 减为 1。然而
// b Rc<List> 不能被回收，因为其引用计数是 1 而不是 0。接下来 Rust 会丢弃 a 将
// a Rc<List> 实例的引用计数从 2 减为 1。这个实例也不能被回收，因为 b Rc<List>
// 实例依然引用它，所以其引用计数是 1。这些列表的内存将永远保持未被回收的状态
