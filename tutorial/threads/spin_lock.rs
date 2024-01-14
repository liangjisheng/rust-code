use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

struct SpinLock {
    lock: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        // 尝试加锁, 如果加锁失败则一直自旋
        {
            // CAS的消耗比较大, 当加锁失败时, 通过简单load读取锁的状态, 只要读取到锁被释放时才会再去尝试CAS加锁
            while self.lock.load(Ordering::Relaxed) {}
        }
    }

    pub fn unlock(&self) {
        // 解锁
        self.lock.store(false, Ordering::Release);
    }
}

fn main() {
    let spinlock = Arc::new(SpinLock::new());
    let spinlock1 = spinlock.clone();

    let thread = thread::spawn(move || {
        // 子线程加锁1，内部调用了compare_exchange 方法，修改状态
        spinlock1.lock();
        println!("do something1!");
        thread::sleep(Duration::from_millis(1000));
        // 子线程解锁1
        spinlock1.unlock();
    });

    thread.join().unwrap();

    // 主线程加锁
    spinlock.lock();
    println!("do something2!");
    // 主线程解锁
    spinlock.unlock();
}
