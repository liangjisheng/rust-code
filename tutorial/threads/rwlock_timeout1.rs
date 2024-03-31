//    引入第三方库处理超时
//    parking_lot = "0.12.1"
use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let rwlock = Arc::new(RwLock::new(0));
    let start = Instant::now();

    // 尝试在 1 秒内获取读锁
    let reader = loop {
        if let Some(r) = rwlock.try_read_for(Duration::from_secs(1)) {
            break r;
        }
        if start.elapsed() >= Duration::from_secs(5) {
            panic!("Failed to acquire read lock within 5 seconds.");
        }
    };

    // 尝试在 1 秒内获取写锁
    let mut writer = loop {
        if let Some(w) = rwlock.try_write_for(Duration::from_secs(1)) {
            break w;
        }
        if start.elapsed() >= Duration::from_secs(5) {
            panic!("Failed to acquire write lock within 5 seconds.");
        }
    };

    // 进行读写操作
    println!("Reader: {}", *reader);
    *writer += 1;
    println!("Writer: {}", *writer);
}
