use std::cell::RefCell;
use std::rc::Rc;

// RefCell 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，
// 从编译器错误变成了 panic 异常

// RefCell 简单总结
// 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
// RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
// RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
// 使用 RefCell 时，违背借用规则会导致运行期的 panic

// 总之，当非要使用内部可变性时，首选 Cell，只有你的类型没有实现 Copy 时，才去选择 RefCell

fn r1() {
    let s = RefCell::new(String::from("hello, world"));
    {
        let s1 = s.borrow();
        println!("s1 {}", s1);
    }
    let mut s2 = s.borrow_mut();
    println!("s2 {}", s2);
    *s2 = "alice".to_string();
    println!("s2 {}", s2);

    let my_cell = RefCell::new(42);
    let my_ref = my_cell.borrow();
    println!("The value in my_cell is: {}", *my_ref);
}

fn r2() {
    let my_cell = RefCell::new(42);
    let mut my_ref = my_cell.borrow_mut();
    *my_ref = 100;
    println!("The new value in my_cell is: {}", *my_ref);
}

// 如果我们在获取 RefCell 的可变引用之前，已经获取了一个不可变引用，那么 Rust 会在运行时检查，
// 如果发现了错误，就会 panic

fn r3() {
    let my_cell = RefCell::new(42);
    let my_ref = my_cell.borrow();
    let my_mut_ref = my_cell.borrow_mut(); // panic!
}

fn r4() {
    let my_cell = Rc::new(RefCell::new(42));

    let my_ref1 = my_cell.borrow();
    let my_ref2 = my_cell.borrow();
    // panic
    // let mut my_mut_ref = my_cell.borrow_mut();
    // *my_mut_ref = 100;

    println!("The value in my_cell is: {}", *my_ref1);
    println!("The value in my_cell is: {}", *my_ref2);
}

fn r5() {
    let my_cell = RefCell::new(0);

    let mut my_mut_ref1 = my_cell.borrow_mut();
    *my_mut_ref1 += 1;
    // panic
    let mut my_mut_ref2 = my_cell.borrow_mut();
    *my_mut_ref2 += 2;

    println!("The value in my_cell is: {}", *my_mut_ref1);
}

fn main() {
    // r1();
    // r2();
    // r3();
    // r4();
    // r5();
}
