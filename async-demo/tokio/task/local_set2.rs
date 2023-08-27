// 除了LocalSet::spawn_local()可以生成新的本地异步任务，tokio::task::spawn_local()
// 也可以生成新的本地异步任务，但是它的使用有个限制，必须在LocalSet上下文内部才能调用

use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    let local_tasks = tokio::task::LocalSet::new();

    local_tasks.spawn_local(async {
        println!("local task1");
        time::sleep(time::Duration::from_secs(1)).await;
        println!("local task1 done");
    });

    local_tasks.spawn_local(async {
        println!("local task2");
        time::sleep(time::Duration::from_secs(2)).await;
        println!("local task2 done");
    });

    println!("before local tasks running: {}", now());
    // LocalSet::block_on进入LocalSet上下文
    local_tasks.block_on(&rt, async {
        tokio::task::spawn_local(async {
            println!("local task3");
            time::sleep(time::Duration::from_secs(3)).await;
            println!("local task3 done");
        })
        .await
        .unwrap();
    });
    println!("all local tasks done: {}", now());
}

// 需要注意的是，调用LocalSet::block_on()和LocalSet::run_until()时均需指定一个
// 异步任务(Future)作为其参数，它们都会立即开始执行该异步任务以及本地任务队列中已存在
// 的任务，但是这两个函数均只等待其参数对应的异步任务执行完成就返回。这意味着，它们返回
// 的时候，可能还有正在执行中的本地异步任务，它们会继续保留在本地任务队列中。当再次进入
// LocalSet上下文或await LocalSet的时候，它们会等待调度并运行
