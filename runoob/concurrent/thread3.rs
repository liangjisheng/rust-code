// move 强制所有权迁移
use std::thread;

fn main() {
    let s = "hello";

    // let handle = thread::spawn(|| {
    //     println!("{}", s);
    // });

    // 强制 s 的所有权转移到子线程
    let handle = thread::spawn(move || {
        println!("{}", s);
    });

    handle.join().unwrap();
}
