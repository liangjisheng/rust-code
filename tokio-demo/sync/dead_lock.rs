// tokio RwLock 实现的是写锁优先，它的具体规则如下：

// 每次申请锁时都将等待，申请锁的异步任务被切换，CPU交还给调度器
// 如果申请的是读锁，并且此时没有写锁存在，则申请成功，对应的任务被唤醒
// 如果申请的是读锁，但此时有写锁(包括写锁申请)的存在，那么将等待所有的写锁释放(因为写锁总是优先)
// 如果申请的是写锁，如果此时没有读锁的存在，则申请成功
// 如果申请的是写锁，但此时有读锁的存在，那么将等待当前正在持有的读锁释放

// 注意，RwLock的写锁优先会很容易产生死锁。例如，下面的代码会产生死锁

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

            time::sleep(Duration::from_secs(2)).await;
            let nn = lock1.read().await;
        });

        time::sleep(Duration::from_secs(1)).await;
        let mut wn = lock.write().await;
        *wn = 2;
    });
}

// 申请第一把读锁时，因为此时无锁，所以读锁(即变量n)申请成功。1秒后申请写锁时，
// 由于此时读锁n尚未释放，因此写锁申请失败，将等待。再1秒之后，继续在子任务中申请读锁，
// 但是此时有写锁申请存在，因此第二次申请读锁将等待，于是读锁写锁互相等待，死锁出现了
