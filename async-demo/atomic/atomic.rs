// 因为原子类型都实现了Sync trait，所以原子类型的变量在线程之间共享是安全的，
// 但因为它们本身没有提供共享机制，因此比较常见的用法是将其放在原子引用计数智能指针Arc中

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // 使用原子类型创建一个锁，通过引用计数获得共享所有权
    let spinlock = Arc::new(AtomicUsize::new(1));
    // 引用计数 +1
    let spinlock_clone = spinlock.clone();

    let thread = thread::spawn(move || {
        // SeqCst排序：写操作（存储）使用release 语义：写屏障之前的读写操作不能重排在写屏障之后
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 使用 while循环，来等待某个临界区可用的一种锁
    // SeqCst排序：读操作（读取）使用 acquire 语义 读屏障之后的读写操作不能重排到读写屏障之前
    // 上面的线程中的写（存储）指令，下面的指令要求之后的读写操作不能在此之前
    while spinlock.load(Ordering::SeqCst) != 0 {}

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}

// 自旋锁是指当一个线程尝试去获取某一把锁的时候，如果这个锁此时已经被其他线程获取，
// 那么此线程就无法获取到这把锁，该线程将会等待，间隔一段时间后会再次尝试获取。
// 自旋锁实际上是通过 CPU 空转 (spin) 忙等待 (busy wait)，例如上面代码中的
// while 循环，来等待某个临界区可用的一种锁
