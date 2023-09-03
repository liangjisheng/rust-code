// tokio::sync::Mutex使用new()来创建互斥锁，使用lock()来申请锁，
// 申请锁成功时将返回MutexGuard，并通过drop的方式来释放锁

// 需特别说明的是，tokio::sync::Mutex其内部使用了标准库的互斥锁，即std::sync::Mutex
// 而标准库的互斥锁是针对线程的，因此，使用tokio的互斥锁时也会锁住整个线程。此外，
// tokio::sync::Mutex是对标准库的Mutex的封装，性能相对要更差一些。也因此，官方文档中建议
// 如非必须，应使用标准库的Mutex或性能更高的parking_lot提供的互斥锁，而不是tokio的Mutex

use std::sync::Arc;
use tokio::{
    self, sync,
    time::{self, Duration},
};

async fn tikio_mutex() {
    let mutex = Arc::new(sync::Mutex::new(0));

    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        tokio::spawn(async move {
            let mut data = lock.lock().await;
            *data += 1;
            println!("task: {}, data: {}", i, data);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}

async fn std_mutex() {
    let mutex = Arc::new(std::sync::Mutex::new(0));

    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        tokio::spawn(async move {
            let mut data = lock.lock().unwrap();
            *data += 1;
            println!("task: {}, data: {}", i, data);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    // tikio_mutex().await;
    std_mutex().await;
}
