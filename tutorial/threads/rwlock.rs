// Mutex会对每次读写都进行加锁，但某些时候，我们需要大量的并发读，
// Mutex就无法满足需求了，此时就可以使用RwLock:

// 对于读写锁，我们需要保证写操作在读操作之前，因此，在调用 write 方法时，会等待所有的读取守卫
// 被释放，并阻止新的读取守卫的创建。为了避免死锁和优先级反转，写入守卫还可以降低优先级。
// 读写锁的实现主要是通过两个 Mutex 来实现的。一个 Mutex 用于保护读取计数器，另一个 Mutex 用于
// 保护写入状态。读取计数器统计当前存在多少个读取锁，每当一个新的读取锁被请求时，读取计数器就会自增。
// 当读取计数器为 0 时，写入锁可以被请求。

// 类似于 Mutex，RwLock 也支持 poisoning 机制。如果 RwLock 发生 panic，那么锁就成了
// poison 状态，也就是无法再被使用。任何试图获取这个锁的线程都会 panic，而不是被阻塞。

use std::sync::RwLock;

fn r1() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        // r1, r2 读取守卫被析构时，RwLock 的读取计数器会减少，如果读取计数器变为 0，则写入锁可以被请求。
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        // w 写入守卫被析构时，写入锁立即被释放，并且所有等待读取锁和写入锁的线程都可以开始运行。
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    } // 写锁在此处被drop
}

fn r2() {
    // try_read() 方法用于非阻塞地获取读锁。如果读锁已经被占用，则返回 None。
    let lock = RwLock::new(0u32);

    if let r = lock.try_read().unwrap() {
        println!("read: {}", *r);
    } else {
        println!("read lock is already taken");
    };

    // try_write() 方法用于非阻塞地获取写锁。如果写锁已经被占用，则返回 None。
    if let mut w = lock.try_write().unwrap() {
        *w += 1;
        println!("write: {}", *w);
    } else {
        println!("write lock is already taken");
    };
}

fn main() {
    // r1();
    r2();
}
