// Rust 并没有异常，而是将错误处理分为可恢复错误(recoverable)和不可恢复错误(unrecoverable)
// 可恢复错误 Result<T, E>
// 不可恢复错误 panic!

// 设置环境变量 RUST_BACKTRACE=1 可以打印异常栈
// 使用 panic::catch_unwind 函数让开发者捕获 Panic，以便程序可以继续执行而不被中止。
// 尽量避免使用panic::catch_unwind，因为可能会导致内存不安全

use std::panic;

fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    let result = panic::catch_unwind(|| println!("{}", v[99]));
    assert!(result.is_err());
    println!("{}", v[1]);
}

// 1
// panic错误信息
// 2
