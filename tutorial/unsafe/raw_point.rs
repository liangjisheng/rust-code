// 解引用裸指针包括两种类型：
// 不可变解引用裸指针：*const T；
// 可变解引用裸指针：*mut T；
// 注意，上面的*是类型名的一部分，而不是解引用运算符

// 解引用裸指针与引用和智能指针的区别在于：
// 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针；
// 不保证指向有效的内存；
// 允许为null；
// 不能实现任何自动清理功能

// 可以认为解引用裸指针就是C语言中最基础的指针
// 创建裸指针并不需要在unsafe中声明(因为没有任何危险)，但是使用的时候必须在unsafe块中使用

fn u1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn r1() {
    // 基于引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 细心的同学可能会发现，在这段代码中并没有 unsafe 的身影，原因在于：创建裸指针
    // 是安全的行为，而解引用裸指针才是不安全的行为

    let mut num = 5;
    let r1 = &num as *const i32;
    unsafe {
        println!("r1 is: {}", *r1);
    }
}

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn r2() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
}

// 使用 * 解引用, 除了使用 as 来显式的转换，我们还使用了隐式的转换方式
// let c: *const i32 = &a;。在实际使用中，我们建议使用 as 来转换，
// 因为这种显式的方式更有助于提醒用户：你在使用的指针是裸指针，需要小心。
fn r3() {
    let a = 1;
    let b: *const i32 = &a as *const i32;
    let c: *const i32 = &a;
    unsafe {
        println!("{}", *c);
    }

    // 基于智能指针创建裸指针
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);
}

// 像之前代码演示的那样，使用裸指针可以让我们创建两个可变指针都指向同一个数据，
// 如果使用安全的 Rust，你是无法做到这一点的，违背了借用规则，编译器会对我们
// 进行无情的阻止。因此裸指针可以绕过借用规则，但是由此带来的数据竞争问题，就
// 需要大家自己来处理了，总之，需要小心！

fn main() {
    // u1();
    // r1();
    r2();
}
