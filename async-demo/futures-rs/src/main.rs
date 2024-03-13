use futures::channel::mpsc;
use futures::executor::block_on;
use futures::future;
use futures::future::{join, join_all, try_join, try_join_all, TryFutureExt};
use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::stream::{self, FusedStream, FuturesUnordered, StreamExt};
use futures::{pin_mut, select};
use futures::{SinkExt, Stream};
use std::io;
use std::pin::Pin;

// async 语句块和 async fn 最大的区别就是前者无法显式的声明返回值，在大多数时候
// 这都不是问题，但是当配合 ? 一起使用时，问题就有所不同:

async fn foo() -> Result<u8, String> {
    Ok(1)
}

async fn bar() -> Result<u8, String> {
    Ok(1)
}

fn q1() {
    let fut = async {
        foo().await?;
        bar().await?;

        // 如果这样写 Ok(()) 则编译会报下面的错误
        // cannot infer type of the type parameter `E` declared on the enum `Result`
        // Ok(())

        Ok::<(), String>(()) // 在这一行进行显式的类型注释
    };
}

// 在多线程章节我们深入讲过 Send 特征对于多线程间数据传递的重要性，对于 async fn
// 也是如此，它返回的 Future 能否在线程间传递的关键在于 .await 运行过程中，作用
// 域中的变量类型是否是 Send。

// 学到这里，相信大家已经很清楚 Rc 无法在多线程环境使用，原因就在于它并未实现 Send
// 特征，那咱就用它来做例子:

use std::rc::Rc;

#[derive(Default, Debug)]
struct NotSend(Rc<()>);

// 事实上，未实现 Send 特征的变量可以出现在 async fn 语句块中:
async fn bar1() {}
async fn foo1() {
    NotSend::default();
    // 即使 foo 返回的 Future 是 Send， 但是在它内部短暂的使用 NotSend 依然是安全的，
    // 原因在于它的作用域并没有影响到 .await，下面来试试声明一个变量，然后让 .await 的
    // 调用处于变量的作用域中试试:

    // 报错
    // let x = NotSend::default();

    bar1().await;
}

fn require_send(_: impl Send) {}

fn q2() {
    require_send(foo1());
}

fn main() {
    // block_on(race_tasks());
}
