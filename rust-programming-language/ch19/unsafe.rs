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

// 在使用Unsafe Rust的代码时，需要在unsafe代码块中执行，当一个方法为不安全的时候，
// 需要将其声明为unsafe，同时调用它时也需要在unsafe中执行
// unsafe函数和方法与常规函数方法十分类似，只是其开头有一个额外的 unsafe
// unsafe函数体也是有效的 unsafe 块，所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块
// 通过 unsafe 块，我们向 Rust 保证了我们已经阅读过函数的文档，理解如何正确使用，并验证过其满足函数的契约。

unsafe fn unsafe_func() {
    println!("unsafe_func");
}

unsafe fn dangerous1() {
    println!("dangerous1");
}

unsafe fn dangerous2() {
    dangerous1();
}

fn u2() {
    unsafe {
        unsafe_func();
        dangerous2();
    }
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // 使用 as_mut_ptr 方法访问 slice 的裸指针
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // 这里使用此函数从 ptr 中创建了一个有 mid 个项的 slice
            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
            // 裸指针上的 add 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。因此必须将
            // slice::from_raw_parts_mut 和 add 放入 unsafe 块中以便能调用它们。通过观察代码，和增加
            // mid 必然小于等于 len 的断言，我们可以说 unsafe 块中所有的裸指针将是有效的 slice 中数据的指针。
            // 这是一个可以接受的 unsafe 的恰当用法。
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 创建外部函数接口(Foreign Function Interface， FFI)，
// 需要在extern块内声明，然后使用时必须在unsafe块中使用。
// 任何extern块中的方法都是不安全的，因为无法保证外部方法一定安全
extern "C" {
    fn abs(input: i32) -> i32;
}

fn u3() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 如果是其他语言调用Rust编写的函数：需要在fn关键字前添加extern关键字，并需要添加
// #[no_mangle]注解来告诉 Rust 编译器不要 改变 此函数的名称
// (因为一般Rust编译器会改变函数名，使其带上更多信息)
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 静态全局变量(不可变)
// 通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法。静态变量只能储存拥有 'static 生命周期的引用，
// 这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。访问不可变静态变量是安全的。
static HELLO_WORLD: &str = "Hello, world!";

// 默认情况下，Rust中的全局变量(静态变量)是不能被修改的，因为在并发环境下修改全局变量会出现数据竞争
// 静态变量只能储存拥有 'static 生命周期的引用
// 常量与不可变静态变量可能看起来很类似，区别是
// 静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。另一方面，常量则允许在任何被用到的时候复制其数据。
// 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 Unsafe 的
// 拥有可以全局访问的可变数据，难以保证不存在数据竞争，这就是为何 Rust 认为可变静态变量是不安全的。
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn u4() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    // u1();
    // u2();
    u4();
}
