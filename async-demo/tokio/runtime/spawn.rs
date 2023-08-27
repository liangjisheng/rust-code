// 使用 tokio::spawn() 来生成异步任务

use std::thread;

use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T.%3f").to_string()
}

// 在runtime 外部定义一个异步任务，且该函数返回值不是 Future 类型
fn async_task() {
    println!("create an async task: {}", now());
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("async task over: {}", now());
    });
}

// 除了tokio::spawn()，runtime自身也能spawn，因此，也可以传递runtime
// (注意，要传递runtime的引用)，然后使用runtime的spawn()
fn async_task1(rt: &Runtime) {
    println!("create an async task1: {}", now());
    rt.spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("async task1 over: {}", now());
    });
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 调用函数，该函数内创建了一个异步任务，将在当前runtime内执行
        async_task();
    });

    rt.block_on(async {
        async_task1(&rt);
    });

    // 阻塞当前线程，等待异步任务的完成
    thread::sleep(std::time::Duration::from_millis(1100));
}
