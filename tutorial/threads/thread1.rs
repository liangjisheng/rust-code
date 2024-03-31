use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 调用thread::spawn方法会返回一个Result，该Result包含一个智能指针，该智能指针拥有对
// 线程的所有权，如果线程执行成功则返回Ok，否则返回Err。通过这个智能指针我们可以管理线程
// 的生命周期和操作线程。

fn t1() {
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

// Thread也支持通过std::thread::Builder结构体进行创建，Builder提供了一些线程的
// 配置项，如线程名字、线程优先级、栈大小等。

fn t2() {
    let thr0 = thread::current();
    let thread_name = thr0.name().unwrap_or("unknown");
    let thread_id = thr0.id();
    println!("当前线程的名称：{}", thread_name);
    println!("当前线程的 id: {:?}", thread_id);

    let builder = thread::Builder::new().name("my_thread".into());
    let handle = builder.spawn(move || {
        let thr = thread::current();
        let name = thr.name().unwrap_or("unknown");
        let thread_id = thr.id();
        println!("当前线程的名称：{}", name);
        println!("当前线程的 id: {:?}", thread_id);
    });
    handle.expect("执行失败").join().unwrap();
}

// Rust中Thread对象表示的是系统中的一个线程，可以通过thread::JoinHandle结构体的
// is_finalized()和thread::Thread的panicking()方法来查看线程是否结束和是否因
// panic 而结束

fn t3() {
    let handle = thread::spawn(|| {
        // TODO: 执行耗费时间的任务
        thread::sleep(Duration::from_secs(1));
    });
    while !handle.is_finished() {
        thread::sleep_ms(100);
    }

    if thread::panicking() {
        println!("线程因panic而结束");
    } else {
        println!("线程正常结束");
    }
}

fn t4() {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        tx.send("Hello Thread!".to_string()).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let msg = rx.recv().unwrap();
        println!("{}", msg);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    // t1();
    // t2();
    // t3();
    t4();
}
