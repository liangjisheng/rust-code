// FFI（Foreign Function Interface）可以用来与其它语言进行交互，但是
// 并不是所有语言都这么称呼，例如 Java 称之为 JNI（Java Native Interface）

// 创建外部函数接口(Foreign Function Interface， FFI)，
// 需要在extern块内声明，然后使用时必须在unsafe块中使用。
// 任何extern块中的方法都是不安全的，因为无法保证外部方法一定安全
extern "C" {
    fn abs(input: i32) -> i32;
}

fn u1() {
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

fn main() {
    u1();
}
