// 需要注意的是，调用LocalSet::block_on()和LocalSet::run_until()时均需指定一个
// 异步任务(Future)作为其参数，它们都会立即开始执行该异步任务以及本地任务队列中已存在
// 的任务，但是这两个函数均只等待其参数对应的异步任务执行完成就返回。这意味着，它们返回
// 的时候，可能还有正在执行中的本地异步任务，它们会继续保留在本地任务队列中。当再次进入
// LocalSet上下文或await LocalSet的时候，它们会等待调度并运行

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
        time::sleep(time::Duration::from_secs(1)).await;
        println!("local task1 done {}", now());
    });

    // task2 要睡眠 3 秒，它将被第一次 local_tasks.block_on 在 2 秒后中断
    local_tasks.spawn_local(async {
        println!("local task2");
        time::sleep(time::Duration::from_secs(3)).await;
        println!("local task2 done, {}", now());
    });

    println!("before local tasks running: {}", now());
    local_tasks.block_on(&rt, async {
        tokio::task::spawn_local(async {
            println!("local task3");
            time::sleep(time::Duration::from_secs(2)).await;
            println!("local task3 done: {}", now());
        })
        .await
        .unwrap();
    });

    // 线程阻塞 4 秒，此时task2睡眠 3 秒的时间已经过去了
    // 当再次进入LocalSet时，task2 将可以直接被唤醒
    thread::sleep(std::time::Duration::from_secs(4));

    // 再次进入LocalSet
    local_tasks.block_on(&rt, async {
        // 先执行该任务，当遇到睡眠1秒的任务时，将出现任务切换，
        // 此时，调度器将调度task2，而此时task2已经睡眠完成
        println!("re enter local set context: {}", now());
        time::sleep(time::Duration::from_secs(1)).await;
        println!("re enter local set context done: {}", now());
    });

    // local_tasks.block_on(&rt, async {
    //     println!("re-enter local set context, and exit context");
    //     println!("task2 will not be scheduled");
    // });

    println!("all local tasks done: {}", now());
}

// 正如上面需要睡眠 3 秒的task2，它会被第一次block_on中断，虽然task2已经不再执行，
// 但是 4 秒之后它的睡眠完成事件已经出现，它可以在下次调度本地任务时直接被唤醒。
// 但注意，唤醒的任务不是直接就可以被执行的，而是放入就绪队列等待调度。
// 这意味着，再次进入上下文时，所指定的Future中必须至少存在一个会引起调度切换的任务，
// 否则该Future以同步的方式运行直到结束都不会给已经被唤醒的任务任何执行的机会。
// 例如，将上面示例中的第二个block_on中的Future参数换成下面的async代码块，task2将不会被调度执行

// local_tasks.block_on(&rt, async {
//     println!("re-enter local set context, and exit context");
//     println!("task2 will not be scheduled");
// });
