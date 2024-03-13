// 在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了
// Ordering::Relaxed用于限定内存顺序了，事实上，该枚举有 5 个成员:

// Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序

// Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远
// 在它之前，但是它后面的操作可能被重排到它前面

// Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之
// 前的操作却有可能被重排到它后面，往往和Release在不同线程中联合使用

// AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。
// 比如你要对一个 atomic 自增 1，同时希望该操作之前和之后的读取或写入
// 操作不会被重新排序

// SeqCst 顺序一致性， SeqCst就像是AcqRel的加强版，它不管原子操作是
// 属于读取还是写入的操作，只要某个线程有用到SeqCst的原子操作，线程中该
// SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，且该
// SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。

// 下面我们以Release和Acquire为例，使用它们构筑出一对内存屏障，防止编译器
// 和 CPU 将屏障前(Release)和屏障后(Acquire)中的数据操作重新排在屏障围
// 成的范围之外:

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100; // A
        }
        READY.store(true, Ordering::Release); // B: 内存屏障 ↑
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {} // C: 内存屏障 ↓

        assert_eq!(100, unsafe { DATA }); // D
    })
}

fn main() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
