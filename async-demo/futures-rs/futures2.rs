#![feature(async_closure)]

// https://blog.csdn.net/wowotuo/article/details/91145819

use futures::executor::block_on;
use std::collections::HashMap;
use std::future::Future;

// async和其他Future是懒惰的：它们在运行之前什么都不做。运行Future的最常见方式是.await它。
// 当在Future上调用.await时，它将尝试运行以完成它。如果Future被阻止，它将让出当前线程。
// 当可以取得更多进展时，执行者将获取 Future并将继续运行，以便.await解决

// #![feature(async_await, await_macro, futures_api)]，具体来说，
// async_await 增加了对 async fn 语法的支持。
// await_macro 增加了对 await! 宏的支持。
// async_closure 增加了对异步闭包的支持。
// futures_api 增加了对 nightly std::future 和 std::task 模块的支持,这些模块定义了核心Future特征和依赖类型

async fn compute() {
    println!("async->");
}

// 上面相当于：[只是相当于]
// fn compute1()-> impl Future<Output = ()>{
//     println!("async->");
// }

async fn hello() {
    println!("hello!");

    // async fn
    async fn world() {
        println!("world!");
    }
    world().await;

    // async block
    let a = async {
        println!("{:?}", "hello world!");
    };
    a.await;

    // async move block
    let code = "shang hai".to_string();
    let mut hp = HashMap::new();
    hp.insert(code, "600036");
    let b = async move {
        println!("hello world, i love {:?}", hp);
    };
    // borrow of moved value: `hp`
    // println!("code:{:?}",hp); // error if uncommented
    b.await;
}

// async move块将获取它引用变量的所有权，允许它活得比目前的范围长，但放弃了与其他代码分享那些变量的能力

fn compute_01() -> impl Future<Output = Result<(), String>> {
    async move {
        for i in 1..10000 {
            let _ = i * 2;
        }
        println!("=>01 it is over!");
        Ok(())
    }
}

async fn compute_02() {
    println!("=>02 is entering....");
    _ = compute_01().await;
    // compute_04().await;
    println!("=>02 is over!");
}

async fn compute_03() {
    println!("=>03 is entering....");
    for i in 1..10000 {
        let _ = i * 2;
    }
    println!("=>03 it is over!");
}

async fn compute_04() {
    println!("=> 04 is entering....");
    let capture = "hello".to_owned();
    let block = async move {
        println!("rust says {capture} from async block");
    };
    block.await;

    // borrow of moved value: `capture`
    // 不能再使用 capture, 因为 move 代码块会获取它的所有权
    // println!("capture {}", capture);
    println!("=>04 it is over!");
}

fn compute_05(value: i32) -> impl Future<Output = i32> {
    let closure = async move |v: i32| {
        compute_03().await;
        v + 1
    };
    closure(value)
}

fn main() {
    // block_on(hello());

    block_on(compute_02());
    block_on(compute_03());
    let val = block_on(compute_05(6));
    println!("val :{:?}", val); // 7
}
