// 共享状态或数据，意味着有多个线程同时访问相同的内存位置，Rust 通过互斥器（锁）来实现共享内存并发原语
// 互斥器（mutex）是 「mutual exclusion」 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据

use std::sync::{Arc, Mutex};
use std::thread;

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

// 在线程间共享 Mutex
// 在多线程之间共享信息，存在多个所有者同时拥有所有权，可以使用 Arc 智能指针来存放Mutex
// Arc 是线程安全的，Rc 是非线程安全的，所以，不能是用 Rc ，Rc 被实现为用于单线程场景
fn m2() {
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

fn main() {
    // m1();
    m2();
}

// 简单总结下：一般Rc<T>/RefCell<T>结合用于单线程内部可变性， Arc<T>/Mutex<T>结合用于多线程内部可变性
