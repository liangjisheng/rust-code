// 信号量可以保证在某一时刻最多运行指定数量的并发任务。
// 使用信号量时，需在初始化时指定信号灯(tokio中的SemaphorePermit)的数量，
// 每当任务要执行时，将从中取走一个信号灯，当任务完成时(信号灯被drop)会归还信号灯。
// 当某个任务要执行时，如果此时信号灯数量为0，则该任务将等待，直到有信号灯被归还。
// 因此，信号量通常用来提供类似于限量的功能。

// 例如，信号灯数量为1，表示所有并发任务必须串行运行，这种模式和互斥锁是类似的。
// 再例如，信号灯数量设置为2，表示最多只有两个任务可以并发执行，如果有第三个任务，
// 则必须等前两个任务中的某一个先完成

use chrono::Local;
use std::sync::Arc;
use tokio::{
    self,
    runtime::Runtime,
    sync::Semaphore,
    time::{self, Duration},
};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    // 只有3个信号灯的信号量
    let semaphore = Arc::new(Semaphore::new(3));

    // 5个并发任务，每个任务执行前都先获取信号灯
    // 因此，同一时刻最多只有3个任务进行并发
    for i in 1..=5 {
        let semaphore = semaphore.clone();
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("{}, {}", i, now());
            time::sleep(Duration::from_secs(1)).await;
        });
    }

    time::sleep(Duration::from_secs(3)).await;
}
