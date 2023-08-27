// block_in_place()的目的和spawn_blocking()类似。区别在于spawn_blocking()
// 会新生成一个blocking thread来执行指定的任务，而block_in_place()是在当前
// worker thread中执行指定的可能会长时间运行或长时间阻塞线程的任务，但是它会先将该
// worker thread中已经存在的异步任务转移到其它worker thread，使得这些异步任务不会被饥饿。
// 显然，block_in_place()只应该在多线程runtime环境中运行，如果是单线程runtime
// block_in_place 会阻塞唯一的那个 worker thread

// task::unconstrained() 则是创建一个不受限制不受调度器管理的异步任务，它将不会参与
// 调度器的协作式调度，可以认为是将这个异步任务暂时脱离了调度管理。这样一来，即便该任务
// 中遇到了本该阻塞而放弃线程的操作，也不会去放弃，而是直接阻塞该线程。
// 因此，unconstrained() 创建的异步任务将会使得同线程的其它异步任务被饥饿。如果确实有
// 这样的需求，建议使用 block_in_place() 或 spawn_blocking()

use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime, task, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();

    // spawn: 向runtime中添加新异步任务
    task::spawn(async {
        time::sleep(time::Duration::from_secs(1)).await;
        println!("task over: {}", now());
    });

    task::spawn(async {
        // task::spawn_blocking() 生成一个 blocking thread 来执行指定的任务
        let join = task::spawn_blocking(|| {
            // do some compute-heavy work or call synchronous code
            "blocking completed"
        });

        let result = match join.await {
            Err(_e) => "err",
            Ok(result) => result,
        };
        println!("result: {}", result);
        assert_eq!(result, "blocking completed");
    });

    // task::yield_now 让当前任务立即放弃CPU，将worker thread交还给调度器，
    // 任务自身则进入调度器的就绪队列等待下次被轮询调度。类似于其它异步系统中的next_tick行为
    // 需注意，调用yield_now()后还需await才立即放弃CPU，因为yield_now本身是一个异步任务
    task::spawn(async {
        task::spawn(async {
            // ...
            println!("spawned task done!")
        });

        // Yield, allowing the newly-spawned task to execute first.
        task::yield_now().await;
        println!("main task done!");
    });

    thread::sleep(time::Duration::from_secs(2));
}
