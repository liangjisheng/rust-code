// 闭包可以通过三种方式捕获其环境，它们直接对应到函数获取参数的三种方式：
// 不可变借用，可变借用和获取所有权。闭包会根据函数体中如何使用被捕获的值决定用哪种方式捕获。

fn c1() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn c2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("After calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

use std::thread;

fn main() {
    // c1();
    // c2();

    // 即使闭包体不严格需要所有权，如果希望强制闭包获取它用到的环境中值的所有权，可以在参数列表前使用 move 关键字。
    // 在将闭包传递到一个新的线程时这个技巧很有用，它可以移动数据所有权给新线程
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("Before defining closure: {:?}", list);
}
