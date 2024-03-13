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

fn u1() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    u1();
}
