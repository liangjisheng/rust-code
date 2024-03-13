// 条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起
// 直到某个条件发生后再继续执行

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

// main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
// 子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
