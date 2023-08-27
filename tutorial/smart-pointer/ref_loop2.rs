// 可视化 strong_count 和 weak_count 的改变
// 让我们通过创建了一个新的内部作用域并将 branch 的创建放入其中，来观察 Rc<Node> 实例的
// strong_count 和 weak_count 值的变化。这会展示当 branch 创建和离开作用域被丢弃时会发生什么

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch),   // 1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf),   // 0
        );
    }

    // 当内部作用域结束时，branch 离开作用域，Rc<Node> 的强引用计数减少为 0，所以其 Node 被丢弃
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );
    // 现在 leaf 又是 Rc<Node> 唯一的引用了
}
