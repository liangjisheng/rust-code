// https://www.runoob.com/rust/rust-ownership.html

// rust 中变量所有权
// 所有权有以下三条规则
// Rust 中的每个值都有一个变量，称为其所有者
// 一次只能有一个所有者。
// 当所有者不在程序运行范围时，该值将被删除

// 基本数据类型有这些
// 所有整数类型，例如 i32 、 u32 、 i64 等
// 布尔类型 bool，值为 true 或 false
// 所有浮点类型，f32 和 f64
// 字符类型 char
// 仅包含以上类型数据的元组（Tuples）

fn main() {
    let x = 5;
    // 仅在栈中的数据的"移动"方式是直接复制
    let y = x;

    // 其中 "hello" 可以认为是类似于长度不确定的数据，需要在堆中存储
    // 在给 s2 赋值时 s1 已经无效了。没错，在把 s1 的值赋给 s2 以后 s1 将不可以再被使用
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1) // 错误！s1 已经失效

    // Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式
    // 进行数据交互。但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式——克隆
    let s3 = String::from("hello");
    // 这里是真的将堆中的 "hello" 复制了一份，所以 s3 和 s3 都分别绑定了一个值，释放的时候也会被当作两个资源
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 如果将变量当作参数传入函数，那么它和移动的效果是一样的
    let s5 = String::from("hello"); // s5 被声明有效

    takes_ownership(s5);
    // s5 的值被当作参数传入函数
    // 所以可以当作 s5 已经被移动，从这里开始已经无效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s5

    // 被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放
    let s6 = gives_ownership(); // gives_ownership 移动它的返回值到 s6
    let s7 = String::from("hello"); // s7 被声明有效
    let s8 = takes_and_gives_back(s7); // s7 被当作参数移动, s8 获得返回值所有权
}

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string // a_string 被当作返回值移出函数
}
