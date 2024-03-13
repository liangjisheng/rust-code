// 共享状态或数据，意味着有多个线程同时访问相同的内存位置，Rust 通过互斥器（锁）来实现共享内存并发原语
// 互斥器（mutex）是 「mutual exclusion」 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, sleep};
use std::time::Duration;

fn m1() {
    let m = Mutex::new(5);
    {
        // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止
        let mut num = m.lock().unwrap();
        *num = 10; // 重新赋值
        println!("num is {}", num);

        // Mutex 是一个智能指针。更准确的说，lock 调用返回一个叫做 MutexGuard 的智能指针
        // 这个智能指针实现了Deref来指向其内部数据；其也提供了一个Drop实现当MutexGuard离开作用域时自动释放锁
    }
    println!("m is {:?}", m);
}

fn m2() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞, 发生了死锁
    // drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    // drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m);
}

// 在线程间共享 Mutex
// 在多线程之间共享信息，存在多个所有者同时拥有所有权，可以使用 Arc 智能指针来存放Mutex
// Arc 是线程安全的，Rc 是非线程安全的，所以，不能是用 Rc ，Rc 被实现为用于单线程场景
fn m3() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 这里使用 Rc 会编译报错，因为 Rc 没有实现 Send 和 Sync trait，不能安全的在线程间传递和共享
        // let counter = Rc::clone(&counter);

        // 将所有权移入线程之前克隆了 Arc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// 简单总结下：
// 一般 Rc<T>/RefCell<T> 结合用于单线程内部可变性
// Arc<T>/Mutex<T> 结合用于多线程内部可变性

// 单线程死锁
fn deadlock() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    let d2 = data.lock();
    // 非常简单，只要你在另一个锁还未被释放时去申请新的锁，就会触发，
    // 当代码复杂后，这种情况可能就没有那么显眼。
}

// 多线程死锁, 当我们拥有两个锁，且两个线程各自使用了其中一个锁，然后试图
// 去访问另一个锁时，就可能发生死锁

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn deadlock1() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.lock().unwrap();
                    // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

// try_lock, 与lock方法不同，try_lock会尝试去获取一次锁，
// 如果无法获取会返回一个错误，因此不会发生阻塞

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX3: Mutex<i64> = Mutex::new(0);
    static ref MUTEX4: Mutex<i64> = Mutex::new(0);
}

fn deadlock2() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX3.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX4.try_lock();
                    println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                    // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX4.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    sleep(Duration::from_millis(10));
                    let guard = MUTEX3.try_lock();
                    println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

fn main() {
    // m1();
    m2();
    // m3();
}
