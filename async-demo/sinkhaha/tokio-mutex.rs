// 在使用 Mutex 等同步原语时，要注意标准库的 MutexGuard 无法跨越 .await
// 所以，此时要使用对异步友好的 Mutex，如tokio::sync::Mutex

// 因为 tokio 实现了 work-stealing 调度，Future 有可能在不同的线程中执行
// 普通的 MutexGuard 编译直接就会出错，所以需要使用 tokio 的 Mutex

use anyhow::Result;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tokio::try_join;

struct DB;

impl DB {
    // 假装在 commit 数据
    async fn commit(&mut self) -> Result<usize> {
        Ok(42)
    }
}

// 运行 cargo run --example tokio-mutex
#[tokio::main]
async fn main() -> Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = Arc::clone(&db1);

    let j1 = tokio::spawn(async move {
        let mut db = db1.lock().await; // 拿到锁

        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
        let affected = db.commit().await?;
        println!("db1: Total affected rows: {}", affected);

        Ok::<_, anyhow::Error>(())
    });

    let j2 = tokio::spawn(async move {
        let mut db = db2.lock().await;
        let affected = db.commit().await?;
        println!("db2: Total affected rows: {}", affected);

        Ok::<_, anyhow::Error>(())
    });

    // 让两个 task 有机会执行完
    // tokio::time::sleep(Duration::from_millis(1)).await;

    // try_join! 等待异步执行完成
    let res = try_join!(j1, j2);
    match res {
        Ok((first, second)) => {
            // do something with the values
        }
        Err(err) => {
            println!("processing failed; error = {}", err);
        }
    }

    Ok(())
}
