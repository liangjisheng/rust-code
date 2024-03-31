// Rust 标准库中的 RwLock 目前是不支持读/写超时功能的。我们可以利用 RwLock 中非阻塞方法
// try_read  和try_write 实现超时的特征。

use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

trait TimeoutRwLock<T> {
    fn read_timeout(&self, timeout: Duration) -> Result<RwLockReadGuard<'_, T>, String> {
        match self.try_read_with_timeout(timeout) {
            Ok(guard) => Ok(guard),
            Err(_) => Err(String::from("timeout")),
        }
    }

    fn try_read_with_timeout(&self, timeout: Duration) -> Result<RwLockReadGuard<'_, T>, ()>;
}

impl<T> TimeoutRwLock<T> for RwLock<T> {
    fn try_read_with_timeout(&self, timeout: Duration) -> Result<RwLockReadGuard<'_, T>, ()> {
        let now = std::time::Instant::now();
        loop {
            match self.try_read() {
                Ok(guard) => return Ok(guard),
                Err(_) => {
                    if now.elapsed() >= timeout {
                        return Err(());
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}

fn main() {
    let lock = Arc::new(RwLock::new(0u32));

    let reader = {
        let lock = lock.clone();
        thread::spawn(
            move || match lock.read_timeout(Duration::from_millis(100)) {
                Ok(guard) => {
                    println!("read: {}", *guard);
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            },
        )
    };

    let writer = {
        let lock = lock.clone();
        thread::spawn(move || {
            sleep(Duration::from_secs(1));
            let mut guard = lock.write().unwrap();
            *guard += 1;
            println!("write: {}", *guard);
        })
    };

    reader.join().unwrap();
    writer.join().unwrap();
}
//    输出结果：
// read: 0
// write: 1
