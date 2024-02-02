// Rust 并没有异常，而是将错误处理分为可恢复错误(recoverable)和不可恢复错误(unrecoverable)
// 可恢复错误 Result<T, E>
// 不可恢复错误 panic!

// 设置环境变量 RUST_BACKTRACE=1 可以打印异常栈
// 使用 panic::catch_unwind 函数让开发者捕获 Panic，以便程序可以继续执行而不被中止。
// 尽量避免使用panic::catch_unwind，因为可能会导致内存不安全

// 要获取到栈回溯信息，你还需要开启 debug 标志，该标志在使用 cargo run 或者 cargo build
// 时自动开启（这两个操作默认是 Debug 运行方式）。同时，栈展开信息在不同操作系统或者 Rust
// 版本上也有所不同。

use std::panic;

fn demo1() {
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    let result = panic::catch_unwind(|| println!("{}", v[99]));
    assert!(result.is_err());
    println!("{}", v[1]);
}

// 1
// panic错误信息
// 2

// 主动抛出异常
fn demo2() {
    panic!("crash and burn");
}

fn main() {
    // demo1();
    demo2();
}

// 线程 panic 后，程序是否会终止？
// 长话短说，如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响
// main 线程。因此，尽量不要在 main 线程中做太多任务，将这些任务交由子线程去做，就算子线
// 程 panic 也不会导致整个程序的结束。

// panic 原理剖析
// 当调用 panic! 宏时，它会

// 1. 格式化 panic 信息，然后使用该信息作为参数，调用 std::panic::panic_any() 函数
// 2. panic_any 会检查应用是否使用了 panic hook，如果使用了，该 hook 函数就会被调用
// （hook 是一个钩子函数，是外部代码设置的，用于在 panic 触发时，执行外部代码所需的功能）
// 3. 当 hook 函数返回后，当前的线程就开始进行栈展开：从 panic_any 开始，如果寄存器或
// 者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
// 4. 展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你
// 可能会遇到被用户标记为 catching 的帧（通过 std::panic::catch_unwind() 函数标记），
// 此时用户提供的 catch 函数会被调用，展开也随之停止：当然，如果 catch 选择在内部调用
// std::panic::resume_unwind() 函数，则展开还会继续。

// 还有一种情况，在展开过程中，如果展开本身 panic 了，那展开线程会终止，展开也随之停止。

// 一旦线程展开被终止或者完成，最终的输出结果是取决于哪个线程 panic：对于 main 线程，操作系统
// 提供的终止功能 core::intrinsics::abort() 会被调用，最终结束当前的 panic 进程；如果是
// 其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 std::thread::join() 进行收集。
