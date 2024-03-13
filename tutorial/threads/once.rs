// 有时，我们会需要某个函数在多线程环境下只被调用一次，例如初始化全局变量，无论是哪个线程先调用
// 函数来初始化，都会保证全局变量只会被初始化一次，随后的其它线程调用就会忽略该函数

use std::sync::Once;
use std::thread;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

// call_once 方法
// 执行初始化过程一次，并且只执行一次。
// 如果当前有另一个初始化过程正在运行，线程将阻止该方法被调用。
// 当这个函数返回时，保证一些初始化已经运行并完成，它还保证由执行的闭包所执行的任何内存写入都能
// 被其他线程在这时可靠地观察到。
