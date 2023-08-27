// tokio提供了另外两个关闭runtime的方式：shutdown_timeout()和shutdown_background()
// 前者会等待指定的时间，如果正在超时时间内还未完成关闭，将强行终止runtime中的所有线程。
// 后者是立即强行关闭 runtime

use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T.%3f").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();

    rt.spawn_blocking(|| {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("blocking thread task over: {}", now());
    });

    let _guard = rt.enter();
    rt.spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("worker thread task over 1: {}", now());
    });

    rt.spawn(async {
        std::thread::sleep(std::time::Duration::from_secs(4));
        println!("worker thread task over 2: {}", now());
    });

    // 先让所有任务运行起来
    std::thread::sleep(std::time::Duration::from_millis(3));

    // 1秒后强行关闭 Runtime
    rt.shutdown_timeout(std::time::Duration::from_secs(1));
    println!("runtime dropped: {}", now());
}

// 需要注意的是，强行关闭Runtime，可能会使得尚未完成的任务的资源泄露。因此，应小心使用强行关闭Runtime的操作
