// 所有权是 Rust 的核心特色之一，它让 Rust 无需垃圾回收即可保证内存安全。
// 我们先记住这样一句话： 在 Rust 里每一个值都有一个唯一的所有者，如果当我们对这个值做一个赋值操作，
// 那么这个值的所有权也将发生转移，当所有者离开作用域，这个值也会随之被销毁
// Rust 永远不会自动创建数据的 “深拷贝”

// 所有权只会发生在堆上分配的数据
// 基本类型的存储都在栈上，因此没有所有权的概念

// Rust 在结尾的 } 处自动调用 drop 函数
fn main() {
    // ptr 指向存放字符串内容的指针，len 是 s1 内容使用了多少字节的内存
    // capacity 是容量表示从操作系统获取了多少字节的内存
    let s1 = String::from("hello");
    println!("{}", s1); // hello

    // 基本数据类型 — 赋值
    // 声明基本数据类型 x 将其赋值给变量 y，由于这是一个已知固定大小的值，因此被放入了栈中，
    // 赋值的过程也是一个拷贝的过程，现在栈中既有 x 也有 y，程序是可以正常执行的
    let x: i32 = 5;
    let y: i32 = x;
    println!("x {}, y {}", x, y); // x 5, y 5

    // 复杂数据类型 — 移动
    let s1 = String::from("hello");
    let s2 = s1;
    // print!("s1 {}, s2 {}", s1, s2); // 编译错误
    // String 类型的数据存储在堆上, 那么赋值时也并不会在堆上拷贝一份
    // 当尝试拷贝被分配的内存时，Rust 会使第一个变量无效，这个过程在 Rust 中称为移动，
    // **可以看作 s1 被移动到了 s2 当 s2 离开作用域时，它就会自动释放自己的内存，
    // 这里也再次证明一点，在 Rust 同一时间每一个值仅有一个所有者

    // 基本数据类型存储在栈上，赋值就是一个拷贝的过程，在堆上的数据当你需要拷贝时，
    // 也是可以通过一个 clone 的通用函数实现的
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 {}, s2 {}", s1, s2); // s1 hello, s2 hello

    // 将值传递给函数与变量赋值类似，值的所有权会发生变化，如下示例所示，
    // 外层 s1 处是会报错的，这在编译阶段会报 borrow of moved value: s1 错误
    let s1 = String::from("hello"); // s1 进入作用域
                                    // s1 的值移动到函数 do_something 里
    do_something(s1);
    // print!("s1: {}", s1); // s1 在这里就不再有效了

    // 解决这个问题的一个办法是转移函数返回值的所有权
    let s1 = String::from("hello");
    let s2 = do_something1(s1);
    println!("s2: {} \n", s2);

    // 通过 引用 简单的实现
    // 符号 & 表示引用，&s1 为我们创建一个指向值 s1 的引用，但并不拥有它，所有权没有发生转移
    // 默认为不可变应用，就是不能通过这个引用改变原值
    let s1 = String::from("hello");
    do_something2(&s1);
    println!("s1: {} \n", s1); // s1: hello

    // 预设变量默认是不可变的，想修改引用的值还需使用可变引用
    // 在特定作用域中数据有且只有一个可变引用，好处是在编译时即可避免数据竞争
    let mut s1 = String::from("hello");
    do_something3(&mut s1);
    print!("s1: {} \n", s1); // s1: hello alice

    let mut val = 1;
    add_one(&mut val);
    println!("val: {}", val);
}

fn do_something(s: String) {
    // s 进入作用域
    println!("do_something->s: {}", s);
} // s 离开作用域会被自动释放掉

fn do_something1(s: String) -> String {
    println!("do_something1->s: {} \n", s);
    s
}

fn do_something2(s: &String) {
    println!("do_something2->s: {} \n", s);
} // s 离开作用域，但其没有拥有引用值的所有权，这里也不会发生什么...

fn do_something3(s: &mut String) {
    s.push_str(" alice");
    println!("do_something3->s: {} \n", s); // do_something3->s: hello alice
}

fn add_one(e: &mut i32) {
    *e += 1;
}
