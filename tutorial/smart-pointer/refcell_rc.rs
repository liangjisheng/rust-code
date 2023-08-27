// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
// RefCell<T> 的一个常见用法是与 Rc<T> 结合。回忆一下 Rc<T> 允许对相同数据有多个所有者
// 不过只能提供数据的不可变访问。如果有一个储存了 RefCell<T> 的 Rc<T> 的话，
// 就可以得到有多个所有者 并且 可以修改的值了

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// 这里创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问。
// 接着在 a 中用包含 value 的 Cons 成员创建了一个 List。
// 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权，
// 而不是将所有权从 value 移动到 a 或者让 a 借用 value。

// 我们将列表 a 封装进了 Rc<T> 这样当创建列表 b 和 c 时，他们都可以引用 a。
// borrow_mut 方法返回 RefMut<T> 智能指针，可以对其使用解引用运算符并修改其内部值

// 这是非常巧妙的！通过使用 RefCell<T>，我们可以拥有一个表面上不可变的 List，
// 不过可以使用 RefCell<T> 中提供内部可变性的方法来在需要时修改数据。
// RefCell<T> 的运行时借用规则检查也确实保护我们免于出现数据竞争
// 有时为了数据结构的灵活性而付出一些性能是值得的
