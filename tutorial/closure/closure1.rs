// https://juejin.cn/post/7205908717887438909

// 闭包是一种匿名函数，可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是
// 它允许捕获调用者作用域中的值（捕获环境中的自由变量）。换句话说，闭包是由函数和
// 与其相关的引用环境组合而成的实体

// 闭包由一个结构体组成，当引用了外部的自由变量时就是有大小的，并且引用的是指针；
// 如果没有引用外部自由变量，就是一个空的结构体，大小就是0。所有闭包名称都是唯一的。
// Rust 闭包底层是用结构体实现的。那么闭包是如何找到函数的？实际上是在内部直接写
// 死了函数指针的地址，这是在编译期完成的操作。

use std::thread;

fn c1() {
    let x = 1;
    let sum = |y| x + y;

    assert_eq!(3, sum(2));
}

// FnOnce 说明闭包只能运行一次。仅实现 FnOnce 特征的闭包在调用时会转移所有权
// 所以不能对已失去所有权的闭包变量进行二次调用
// 但是如果闭包实现了 Copy 特征，那么调用时使用的将是拷贝，并没有发生所有权的转移
// 所以可以多次调用

// 此处声明闭包func实现了 Copy，闭包没有发生所有权转移所有func调用两次不崩溃
fn fn_once<F: FnOnce(usize) -> bool + Copy>(func: F) {
    println!("{}", func(3)); // true
    println!("{}", func(4)); // false
}

fn c2() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
}

// 如果想强制转移变量的所有权(重点理解捕获变量x所有权与闭包func所有权的区别)到闭包内部
// 可以在参数列表前添加 move 关键字，通常要求闭包生命周期大于捕获变量声明周期场景
// 如将闭包返回或移入其他线程
fn c3() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // move 关键字转移环境中的变量 v 到线程内部
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // value borrowed here after move
    // println!("{:?}", v);
}

// FnMut 表示以可变引用（&mut T）的方式捕获了环境中的值，因此可以修改该值。
// 要实现可变借用捕获变量，需要将该闭包声明为可变类型，把闭包当做一个普通变量

// 期望函数传参进来的闭包实现的是`FnMut`
fn exec<'a, F: FnMut(&'a str)>(mut func: F) {
    func(" world")
}

fn c4() {
    let mut s = String::new();
    // 闭包仅对环境中变量s进行了可变借用:&mut s
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s); // "hello"

    // 闭包对环境中变量s进行了可变借用: &mut s，即闭包是FnMut特征
    let update_string = |str| s.push_str(str);
    exec(update_string); // 将`FnMut`闭包作为函数传入参数示例：
    println!("{:?}", s); // "hello world"
}

// 期望传参进来一个Fn特征的闭包
fn exec1<'a, F: Fn(String) -> ()>(func: F) {
    // 对于不可变借用闭包，仅将其标记为Fn特征即可
    func("world".to_string())
}

// Fn 表示以不可变引用（&T）的方式捕获环境中的值
fn c5() {
    let s = "hello".to_string();
    // 闭包对环境中变量s进行了不可变借用，相当于&s，即闭包是Fn特征的
    let update_string = |str| println!("{},{}", s, str);
    exec1(update_string);
    println!("{}", s);
}

// 可以在闭包的参数列表前使用move关键字，这样将强制闭包以获取所有权的方式捕获其环境中的变量
// 不过使用了 move 的闭包依然可能实现了Fn和FnMut，因为一个闭包实现了哪种 Fn 特征取决于
// 该闭包内部如何使用被捕获的变量，而不是取决于闭包如何捕获它们。使用的move关键字，强调的
// 就是“闭包如何捕获变量”

fn exec2<F: FnOnce()>(f: F) {
    f()
}

fn c6() {
    let s = String::new();
    // 闭包不仅仅实现了 FnOnce 特征，还实现了 Fn 特征（因为该闭包对于s的使用仅仅是不可变引用）
    // 因此将 exec2 中的 FnOnce 修改为 Fn 也是可以编译通过的
    let update_string = move || println!("{}", s);
    exec2(update_string);
}

fn main() {
    // c1();
    // c2();
    // c3();
    // c4();
    // c5();
    c6();
}
