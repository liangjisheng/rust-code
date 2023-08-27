// 下面是使用run_until()两次进入LocalSet上下文的示例，和block_on()类似
// 区别仅在于它只能在Runtime::block_on()内或[tokio::main]注解的main函数内部被调用

use std::thread;

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
        time::sleep(time::Duration::from_secs(2)).await;
        println!("local task1 done {}", now());
    });

    println!("before local tasks running: {}", now());
    rt.block_on(async {
        local_tasks
            .run_until(async {
                println!("local task2");
                time::sleep(time::Duration::from_secs(1)).await;
                println!("local task2 done: {}", now());
            })
            .await;
    });

    thread::sleep(std::time::Duration::from_secs(3));
    rt.block_on(async {
        local_tasks
            .run_until(async {
                println!("local task3");
                tokio::task::yield_now().await;
                println!("local task3 done: {}", now());
            })
            .await;
    });
    println!("all local tasks done: {}", now());
}
