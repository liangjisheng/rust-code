// Weak 通过 use std::rc::Weak 来引入，它具有以下特点:

// 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
// 可由 Rc<T> 调用 downgrade 方法转换成 Weak<T>
// Weak<T> 可使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 的值是 None
// 常用于解决循环引用的问题

use std::rc::Rc;

fn main() {
    // 创建Rc，持有一个值5
    let five = Rc::new(5);

    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);

    // Weak引用的资源依然存在，取到值5
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    // 手动释放资源`five`
    drop(five);

    // Weak引用的资源已不存在，因此返回None
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(strong_five, None);
}
