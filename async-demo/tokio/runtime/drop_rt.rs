// 关闭Runtime时，将使得该Runtime中的所有异步任务被移除。完整的关闭过程如下：
// 1.先移除整个任务队列，保证不再产生也不再调度新任务
// 2.移除当前正在执行但尚未完成的异步任务，即终止所有的 worker thread
// 3.移除Reactor，禁止接收事件通知

// 注意，这种删除 runtime 句柄的方式只会立即关闭未被阻塞的 worker thread
// 那些已经运行起来的blocking thread以及已经阻塞整个线程的 worker thread
// 仍然会执行。但是，删除runtime又要等待runtime中的所有异步和非异步任务
// (会阻塞线程的任务)都完成，因此删除操作会阻塞当前线程

use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T.%3f").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    // 一个运行5秒的blocking thread
    // 删除rt时，该任务将继续运行，直到自己终止
    rt.spawn_blocking(|| {
        thread::sleep(std::time::Duration::from_secs(3));
        println!("blocking thread task over: {}", now());
    });

    // 进入runtime，并生成一个运行3秒的异步任务，
    // 删除rt时，该任务直接被终止
    let _guard = rt.enter();
    rt.spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("worker thread task over 1: {}", now());
    });

    // 进入runtime，并生成一个运行4秒的阻塞整个线程的任务
    // 删除rt时，该任务继续运行，直到自己终止
    rt.spawn(async {
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("worker thread task over 2: {}", now());
    });

    // 先让所有任务运行起来
    std::thread::sleep(std::time::Duration::from_millis(3));

    // 删除 runtime 句柄，将直接移除那个3秒的异步任务，
    // 且阻塞3秒，直到所有已经阻塞的 thread 完成
    drop(rt);
    println!("runtime dropped: {}", now());
}

// 关闭runtime可能会被阻塞，因此，如果是在某个runtime中关闭另一个runtime，将会导致当前的
// runtime的某个worker thread被阻塞，甚至可能会阻塞很长时间，这是异步环境不允许的
