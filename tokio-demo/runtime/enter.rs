// block_on()是进入runtime的主要方式。但还有另一种进入runtime的方式：enter()
// block_on()进入runtime时，会阻塞当前线程，enter()进入runtime时，不会阻塞
// 当前线程，它会返回一个EnterGuard。EnterGuard没有其它作用，它仅仅只是声明从它
// 开始的所有异步任务都将在runtime上下文中执行，直到删除该EnterGuard。
// 删除EnterGuard并不会删除runtime，只是释放之前的runtime上下文声明。因此，删除
// EnterGuard之后，可以声明另一个EnterGuard，这可以再次进入runtime的上下文环境

use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T.%3f").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();

    // 进入runtime，但不阻塞当前线程
    let guard1 = rt.enter();

    // 生成的异步任务将放入当前的runtime上下文中执行
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("task1 sleep over: {}", now());
    });

    // 释放runtime上下文，这并不会删除runtime
    drop(guard1);

    // 可以再次进入runtime
    let guard2 = rt.enter();
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("task2 sleep over: {}", now());
    });

    drop(guard2);

    // 阻塞当前线程，等待异步任务的完成
    thread::sleep(std::time::Duration::from_secs(2));
}
