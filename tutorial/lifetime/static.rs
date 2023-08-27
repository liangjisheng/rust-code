// 'static，拥有该生命周期的引用可以和整个程序活得一样久
// 字符串变量全部具有 'static 的生命周期
// 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹

// 到底是 &'static 这个引用还是该引用指向的数据活得跟程序一样久呢？
// 答案是引用指向的数据，而引用本身是要遵循其作用域范围的

use std::fmt::Display;

fn print_author(author: &'static str) {
    println!("{}", author);
}

fn print<T: Display + 'static>(message: &T) {
    println!("{}", message);
}

// &'static 生命周期针对的仅仅是引用，而不是持有该引用的变量
// 对于变量来说，还是要遵循相应的作用域规则

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn get_memory_location() -> (usize, usize) {
    // hello 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 string 的生命周期就不一样了，它完全取决于变量作用域
    // 对于该例子来说，也就是当前的函数范围
    let string = "hello";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // string 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 unsafe 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn st1() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 2;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1,r2 持有的数据都是 'static 的, 因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // 2
    println!("&'static str: {}", r2); // &'static str

    let r3: &str;
    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期, 但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱
        static_bound(&s1);

        r3 = &s1;

        // s1 在这里被 drop
    }
    // println!("{}", r3);
}

fn static_bound<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn main() {
    let s1 = "alice";
    print_author(s1);
    print(&s1);
    println!();

    let (pointer, length) = get_memory_location();
    let msg = get_str_at_location(pointer, length);
    println!("The {} bytes at 0x{:X} stored: {}", length, pointer, msg);
    println!();

    // let msg = get_str_at_location(1000, 10);
    // println!("{}", msg);

    let i = 5;
    // error: borrowed value does not live long enough
    // &i 的生命周期是当前函数, 不满足 'static
    // print_it(&i);
    // 同上面一样的报错
    // print_it1(&i);

    // 这段代码竟然不报错了！原因在于我们约束的是 T, 但是使用的却是它的引用 &T
    // 换而言之, 我们根本没有直接使用 T, 因此编译器就没有去检查 T 的生命周期
    // 它只要确保 &T 的生命周期符合规则即可
    print_it2(&i);
    println!();

    st1();
}
