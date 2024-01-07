// std::time也提供了sleep()，但它会阻塞整个线程，而tokio::time中的sleep()
// 则只是让它所在的任务放弃CPU并进入调度队列等待被唤醒，它不会阻塞任何线程，
// 它所在的线程仍然可被用来执行其它异步任务。因此，在tokio runtime中，应当使用
// tokio::time中的sleep()

use chrono::Local;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    // block_on 会阻塞当前线程(main函数所在的主线程), 直到其指定的**异步任务树(可能有子任务)**全部完成
    // 所以第二个 block_on, 要等第一个执行完后才能执行
    rt.block_on(async {
        println!("before sleep: {}", Local::now().format("%F %T.%3f"));
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("after sleep: {}", Local::now().format("%F %T.%3f"));
    });

    // block_on 也有返回值，其返回值为其所执行异步任务的返回值
    let res: i32 = rt.block_on(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        3
    });
    println!("{}", res);
}
