use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，
    // 当对其调用 join 方法时，它会等待其线程结束。
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 如果 handle 在这里 join 的话
    // 主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    // 阻塞（Blocking）线程意味着阻止该线程执行工作或退出。
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // 通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
