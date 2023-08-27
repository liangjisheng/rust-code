#![feature(async_closure)]

// async 关键字可以用于创建如下类型的 Future：
// 定义函数：async fn
// 定义 block： async {}

// Future 不会立即执行，要想执行 Future 定义的函数，需要：
// 使用 await
// 或者在异步运行时中为该 Future 创建 task

// 创建异步任务，可以选择如下方式：
// 使用 block_on
// 使用 spawn

use async_std;
use futures;
use std::time;

#[derive(Debug)]
struct Song {
    name: String,
}

// 异步函数
// async fn foo(args..) -> T 的实际类型签名是 fn(args..) -> impl Future<Output = T>
// 其中的返回的类型是由编译器生成的匿名类型
async fn learn_song() -> Song {
    println!("start learn song");
    async_std::task::sleep(time::Duration::from_secs(2)).await;
    // 如果使用 thread::sleep(), 则会使得 f1, f2 不会并发执行
    // 因为这会阻塞整个线程
    // thread::sleep(time::Duration::from_secs(3));
    Song {
        name: "See you again".to_string(),
    }
}

async fn sing_song(song: Song) {
    println!("sing song {}", song.name);
}

async fn dance() {
    async_std::task::sleep(time::Duration::from_secs(1)).await;
    // thread::sleep(time::Duration::from_secs(1));
    println!("doing dance");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

// await! 是一个编译器内建的 macro ，用来暂停（pause） Future
// 的执行流程，并且把执行流程交回给调用方 (yield)
// await! 只能用于异步函数，异步闭包或者异步块内部，否则将导致编译错误
async fn af1() {
    // 异步闭包返回类型是 impl Future<Output = T>
    let closure = async || {
        async_std::task::sleep(time::Duration::from_millis(200)).await;
        println!("Hello from async closure.");
    };
    let future = closure();

    // 通过异步块可以便捷地创建一个 Future
    let future1 = async {
        async_std::task::sleep(time::Duration::from_millis(100)).await;
        println!("Hello from an async block");
        1
    };

    // 加上 await 后, future, future1 变成线性执行
    future.await;
    // let n = await!(future);
    println!("n {}", future1.await);

    // 下面这句可以并发执行
    // futures::join!(future, future1);
}

async fn foo(i: u32) -> u32 {
    10 + i
}

async fn af2() {
    let futures = vec![foo(1), foo(2), foo(3)];
    assert_eq!(futures::future::join_all(futures).await, [11, 12, 13]);

    let fs = futures::future::join(foo(1), foo(2));
    println!("{:?}", fs.await);

    // join! 返回异步函数返回值
    let fs1 = futures::join!(foo(1), foo(2));
    println!("{:?}", fs1);
}

fn main() {
    // futures::executor::block_on(async_main());
    // futures::executor::block_on(af1());
    futures::executor::block_on(af2());
}
