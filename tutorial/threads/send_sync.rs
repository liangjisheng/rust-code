// Send和Sync是 Rust 安全并发的重中之重，但是实际上它们只是标记特征
// (marker trait，该特征未定义任何行为，因此非常适合用于标记)

// 实现Send的类型可以在线程间安全的传递其所有权
// 实现Sync的类型可以在线程间安全的共享(通过引用)

// 在 Rust 中，几乎所有类型都默认实现了Send和Sync，而且由于这两个特征都是可自动
// 派生的特征(通过derive派生)，意味着一个复合类型(例如结构体), 只要它内部的所有
// 成员都实现了Send或者Sync，那么它就自动实现了Send或Sync。

// 裸指针两者都没实现，因为它本身就没有任何安全保证
// UnsafeCell不是Sync，因此Cell和RefCell也不是
// Rc两者都没实现(因为内部的引用计数器不是线程安全的)

// 手动实现 Send 和 Sync 是不安全的，通常并不需要手动实现 Send 和 Sync trait，
// 实现者需要使用unsafe小心维护并发安全保证。

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// 为裸指针实现Send
#[derive(Debug)]
struct MyBox(*mut u8);
// Send和Sync是unsafe特征，实现时需要用unsafe代码块包裹
unsafe impl Send for MyBox {}

fn s1() {
    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });

    t.join().unwrap();
}

#[derive(Debug)]
struct MyBox1(*const u8);
unsafe impl Send for MyBox1 {}
unsafe impl Sync for MyBox1 {}

fn s2() {
    let b = &MyBox1(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();
}

fn main() {
    // s1();
    s2();
}

// 通过上面的两个裸指针的例子，我们了解了如何实现Send和Sync，以及如何只实现Send
// 而不实现Sync，简单总结下：

// 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
// 绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc 等
// 可以为自定义类型实现Send和Sync，但是需要unsafe代码块
// 可以为部分 Rust 中的类型实现Send、Sync，但是需要使用 newtype，例如文中的裸指针例子
