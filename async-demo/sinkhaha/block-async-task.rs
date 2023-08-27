use anyhow::Result;
use std::time::Duration;

// 运行 cargo run --example blocl-async-task

// 强制 tokio 只使用一个工作线程，这样 task 2 不会跑到其它线程执行
#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
    // 先开始执行 task 1 的话会阻塞，让 task 2 没有机会运行
    tokio::spawn(async move {
        eprintln!("task 1");

        // 试试把这句注释掉看看会产生什么结果
        // tokio::time::sleep(Duration::from_millis(1)).await;

        // 死循环
        loop {}
    });

    tokio::spawn(async move {
        eprintln!("task 2");
    });

    println!("main thread");
    tokio::time::sleep(Duration::from_millis(1)).await;
    Ok(())
}

// 这段代码的 task 2 会没有机会执行到，因为 task 1 有一个死循环
// task 1 不执行结束（不让出 CPU），task 2 就没有机会被调度
