// 避免引用循环：将 Rc<T> 变为 Weak<T>
// 到目前为止，我们已经展示了调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其
// strong_count 为 0 时才会被清理的 Rc<T> 实例。你也可以通过调用 Rc::downgrade 并传递
// Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。强引用代表如何共享 Rc<T> 实例的所有权。
// 弱引用并不属于所有权关系，当 Rc<T> 实例被清理时其计数没有影响。他们不会造成引用循环，
// 因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

// 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。不同于将 Rc<T> 实例的 strong_count 加 1，
// 调用 Rc::downgrade 会将 weak_count 加 1。Rc<T> 类型使用 weak_count 来记录其存在多少个
// Weak<T> 引用，类似于 strong_count。其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

// 强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，
// 因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

// 因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
// 为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，
// 则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。因为 upgrade 返回一个 Option<Rc<T>>，
// Rust 会确保处理 Some 和 None 的情况，所以它不会返回非法指针

use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

// parent 的类型不能是 Rc<T>, 因为这样 leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针
// 这会形成引用循环，会造成其 strong_count 永远也不会为 0
// 所以 parent 使用 Weak<T> 类型而不是 Rc<T>，具体来说是 RefCell<Weak<Node>>

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
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Rc::downgrade 函数来从 branch 中的 Rc<Node> 值创建了一个指向 branch 的 Weak<Node> 引用
    // leaf 使用 Weak<T> 弱引用指向了父节点
    // 没有无限的输出表明这段代码并没有造成引用循环
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
