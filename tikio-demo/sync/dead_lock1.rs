// 当要使用写锁时，如果要避免死锁，一定要保证同一个任务中的任意两次锁申请之间，前面已经无锁，并且写锁尽早释放。

// 对于上面的示例，同一个子任务中申请两次读锁，但是第二次申请读锁时，第一把读锁仍未释放，
// 这就产生了死锁的可能。只需在第二次申请读锁前，将第一把读锁释放即可。更完整一点，
// 在写锁写完数据后也手动释放写锁(上面的示例中写完就退出，写锁会自动释放，因此无需手动释放)

use std::sync::Arc;
use tokio::{
    self,
    runtime::Runtime,
    sync::RwLock,
    time::{self, Duration},
};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let lock = Arc::new(RwLock::new(0));

        let lock1 = lock.clone();
        tokio::spawn(async move {
            let n = lock1.read().await;
            drop(n); // 在申请第二把读锁前，先释放第一把读锁

            time::sleep(Duration::from_secs(2)).await;
            let nn = lock1.read().await;
            drop(nn);
        });

        time::sleep(Duration::from_secs(1)).await;
        let mut wn = lock.write().await;
        *wn = 2;
        drop(wn);
    });
}
