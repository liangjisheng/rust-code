use std::cell::RefCell;

// RefCell 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，
// 从编译器错误变成了 panic 异常

fn main() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}

// RefCell 简单总结
// 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
// RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
// RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
// 使用 RefCell 时，违背借用规则会导致运行期的 panic

// 总之，当非要使用内部可变性时，首选 Cell，只有你的类型没有实现 Copy 时，才去选择 RefCell
