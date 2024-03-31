// 如果你想在多个线程之间共享一个 RwLock 对象，就需要使用 Arc
// （atomic reference counting，原子引用计数）来包装它：

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(0u32));

    let readers = (0..6)
        .map(|_| {
            let lock = lock.clone();
            thread::spawn(move || {
                let guard = lock.read().unwrap();
                println!("read: {}", *guard);
            })
        })
        .collect::<Vec<_>>();

    let writers = (0..2)
        .map(|_| {
            let lock = lock.clone();
            thread::spawn(move || {
                let mut guard = lock.write().unwrap();
                *guard += 1;
                println!("write: {}", *guard);
            })
        })
        .collect::<Vec<_>>();

    for reader in readers {
        reader.join().unwrap();
    }
    for writer in writers {
        writer.join().unwrap();
    }
}
