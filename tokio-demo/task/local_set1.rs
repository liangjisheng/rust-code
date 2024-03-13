// tokio 提供了让某些任务固定在某一个线程中运行的功能，叫做 LocalSet
// 这些异步任务被放在一个独立的本地任务队列中，它们不会跨线程执行。
// 要使用tokio::task::LocalSet，需使用LocalSet::new()先创建好一个LocalSet实例，
// 它将生成一个独立的任务队列用来存放本地异步任务。
// 之后，便可以使用 LocalSet 的 spawn_local() 向该队列中添加异步任务。但是，添加的
// 异步任务不会直接执行，只有对 LocalSet 调用 await 或调用 LocalSet::run_until()
// 或 LocalSet::block_on() 的时候，才会开始运行本地队列中的异步任务。调用后两个
// 方法会进入 LocalSet 的上下文环境

use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    let local_tasks = tokio::task::LocalSet::new();

    // 向本地任务队列中添加新的异步任务，但现在不会执行
    local_tasks.spawn_local(async {
        println!("local task1");
        time::sleep(time::Duration::from_secs(1)).await;
        println!("local task1 done");
    });

    local_tasks.spawn_local(async {
        println!("local task2");
        time::sleep(time::Duration::from_secs(1)).await;
        println!("local task2 done");
    });

    println!("before local tasks running: {}", now());
    rt.block_on(async {
        // 开始执行本地任务队列中的所有异步任务，并等待它们全部完成
        local_tasks.await;
    });
}
