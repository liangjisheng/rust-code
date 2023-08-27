// 生成器是rust协程(异步编程)实现的基础，async代码最终会编译成生成器，一个生成器就是
// 一个可恢复函数,生成器和闭包比较相似,但在编译器中会将生成器编译成截然不同的语义。
// 生成器最大的特点就是，程序的执行流程可以在生成器和调用者之间来回切换。当我们需要暂时
// 从生成器中返回的时候，就使用yield关键字；当调用者希望再次进入生成器的时候，就调用
// resume 方法，这时程序执行的流程是从上次yield返回的那个点继续执行 ,它可以保证代码
// 不会重复执行,恢复执行时能获取到上一次变量的值

#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn g1() {
    let mut generator = || {
        yield 1;
        return "foo";
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(1) => {
            println!("yield 1");
        }
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete("foo") => {
            println!("foo");
        }
        _ => panic!("unexpected value from resume"),
    }

    // 生成器和闭包类似,只不过多了一个yield关键词
    // 可以多次调用resume ，这又和迭代器类似(惰性)
    // 和闭包一样可以捕获外部环境的变量，也可以使用move关键字将变量所有权移动到生成器中
    // 每当生成器被drop时，它将drop掉所有捕获的环境变量
    // 生成器自动实现了Send和Sync,但不会自动实现Copy或Clone之类的trait
}

fn g2() {
    // 在编译器中，生成器编译成状态机。每个yield表达式对应一个不同的状态
    let ret = "foo";
    let mut generator = move || {
        yield 1;
        return ret;
    };

    let v = Pin::new(&mut generator).resume(());
    println!("{:?}", v);
    let v = Pin::new(&mut generator).resume(());
    println!("{:?}", v);
}

fn main() {
    // g1();
    g2();
}
