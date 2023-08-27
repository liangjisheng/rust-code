use std::thread;
use std::time::Duration;

fn main() {
    // 调用 thread::spawn函数并传递一个闭包，创建一个新线程
    // 新线程虽然创建了10次循环，但也执行了5次就退出了，当主线程结束时，
    // 新线程也会结束，而不管其是否执行完毕
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("this is thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for k in 1..5 {
        println!("this is main {}", k);
        thread::sleep(Duration::from_millis(1));
    }

    // 阻塞主线程退出，直到新线程执行完毕
    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束
    // 阻塞（Blocking） 线程意味着阻止该线程执行工作或退出
    let _ = handler.join().unwrap();

    // 通过使用move闭包来实现，把主线程的变量所有权转移到闭包里
    let v = vec![2, 4, 5];
    let handler = thread::spawn(move || {
        println!("v is {:?}", v);
    });
    let _ = handler.join().unwrap();

    // 主线程就不能再使用变量 v
    // borrow of moved value: `v`
    // println!("v {:?}", v);
}
