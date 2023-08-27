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
// unsafe函数和方法与常规函数方法十分类似，只是其开头有一个额外的 unsafe。
// unsafe函数体也是有效的 unsafe 块，所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块

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

// 默认情况下，Rust中的全局变量(静态变量)是不能被修改的，因为在并发环境下修改全局变量会出现数据竞争
// 静态变量只能储存拥有 'static 生命周期的引用
// 常量与不可变静态变量可能看起来很类似，区别是
// 静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。另一方面，常量则允许在任何被用到的时候复制其数据。
// 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 Unsafe 的
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
