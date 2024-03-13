// https://zhuanlan.zhihu.com/p/606991033

// futures-rs 提供的许多函数、宏，比如常用的join!、select!等

use async_std;
use futures::executor::block_on;
use futures::future::{self, BoxFuture, FusedFuture, FutureExt, TryFutureExt};
use futures::future::{join, join_all, try_join, try_join_all};
use futures::future::{select, select_all, try_select, Either};
use futures::future::{select_ok, try_join3};
use futures::stream::{self, FusedStream, FuturesUnordered, StreamExt};
use futures::{pin_mut, select, Stream};
use std::ops::Index;
use std::time::{self, Duration};

async fn f_select() {
    // select 返回的新Future将等待两个Future中的任何一个完成，Future需要是Unpin
    // 返回的Future为Either枚举，Either::Left表示左边的先完成，
    // 内部元组为(<左边Future返回值>,<右边Future>)，Either::Right表示右边的先完成，
    // 内部元组为(<右边Future返回值>,<左边Future>)

    let future1 = async { 1 };
    let future2 = async { true };

    // 因为 select 要求 Future 是Unpin的，所以可以先使用 pin_mut! 固定住，
    // 固定后 Future 类似变成了 Pin<&mut impl Future>，它实现了 Unpin
    pin_mut!(future1);
    pin_mut!(future2);

    let either = select(future1, future2).await;
    match either {
        Either::Left((future1_result, future2)) => {
            println!("future1");
            assert_eq!(future1_result, 1);
            assert_eq!(future2.await, true);
        }
        Either::Right((future2_result, future1)) => {
            println!("future2");
            assert_eq!(future2_result, true);
            assert_eq!(future1.await, 1);
        }
    }
}

async fn f_try_select() {
    // try_select 是select的TryFuture版本，两个Future参数均要求是TryFuture, 于是就有4种结果
    let mut future1 = async { Ok::<usize, ()>(1) };
    let mut future2 = async { Ok::<bool, ()>(true) };
    pin_mut!(future1);
    pin_mut!(future2);

    let either = try_select(future1, future2).await;
    match either {
        Ok(Either::Left((future1_result, future2))) => {
            println!("future1");
            assert_eq!(future1_result, 1);
            assert_eq!(future2.await, Ok(true));
        }
        Ok(Either::Right((future2_result, future1))) => {
            println!("future2");
            assert_eq!(future2_result, true);
            assert_eq!(future1.await, Ok(1));
        }
        Err(Either::Left((future1_result, future2))) => {
            println!("future1 err");
            assert_eq!(future1_result, ());
            assert_eq!(future2.await, Ok(true));
        }
        Err(Either::Right((future2_result, future1))) => {
            println!("future2 err");
            assert_eq!(future2_result, ());
            assert_eq!(future1.await, Ok(1));
        }
    }
}

async fn f_select_all() {
    // select_all与select类似，但是接受多个Future，所以Future需要是同一类型
    // select_all 有局限性，比如下面示例，因为Future需要时Unpin，
    // 在栈上就固定就比较麻烦，丧失了vec的动态特性
    // 当需要在栈上就固定时，还不如用 select 或者 select! 宏

    async fn foo(i: u32) -> u32 {
        i
    }
    let foo1 = foo(1);
    let foo2 = foo(2);

    pin_mut!(foo1);
    pin_mut!(foo2);
    let futures = vec![foo1, foo2];
    let (result, index, remaining_futures_) = select_all(futures).await;
    println!("index: {}，value：{}", index, result);
    for f in remaining_futures_.into_iter() {
        println!("{}", f.await);
    }

    // 当固定到堆上时，就没有上面的问题，虽然这样损失了性能
    fn box_foo(i: u32) -> BoxFuture<'static, u32> {
        async move { i }.boxed()
    }

    let mut futures = (0..50).into_iter().map(|i| box_foo(i)).collect();
    loop {
        let (result, index, remaining_futures) = select_all(futures).await;
        println!("index: {}，value：{}", index, result);
        futures = remaining_futures;
        if futures.is_empty() {
            break;
        }
    }

    // 还有一种方法就是使用 FuturesOrdered、FuturesUnordered 替代 select_all，它们更加的灵活
    async fn foo_unordered(i: u32) -> u32 {
        i
    }

    let mut futures: FuturesUnordered<_> =
        (50..100).into_iter().map(|i| foo_unordered(i)).collect();
    while let Some(value) = futures.next().await {
        println!("value: {:?}", value);
    }
}

async fn f_select_ok() {
    // select_ok是select_all的TryFuture版本，接收多个TryFuture
    // 等待任何其中任意一个完成且成功，如果没有成功的，就返回最后一个错误

    fn foo(i: u32) -> BoxFuture<'static, Result<u32, u32>> {
        async move {
            if i > 10 {
                Ok(i)
            } else {
                Err(i)
            }
        }
        .boxed()
    }

    // 一共有 50 个 future
    let mut futures: Vec<BoxFuture<'static, Result<u32, u32>>> =
        (0..50).into_iter().map(|i| foo(i)).collect();
    println!("len {}", futures.len()); // 50

    loop {
        let r = select_ok(futures).await;
        match r {
            Ok((value, remaining_futures)) => {
                println!("value: {}", value);
                futures = remaining_futures;
                if futures.is_empty() {
                    break;
                }
            }
            Err(err) => {
                println!("err: {}", err);
                break;
            }
        }
    }
}

async fn f_select1() {
    // select!、select_biased!
    // select!会同时poll多个Future和Stream，执行第一个完成的Future分支
    // 如果多个Future已准备好，则在运行时伪随机选择其中一个，select_biased!则是按照代码的顺序
    // select!要求
    // Future必须是Unpin，如果将产生Future的表达式传递给select
    // 不需要显示 Unpin，因为select!会自动 pin_mut!
    // Future必须是FusedFuture，以保证Future准备好值后不再被poll
    // 可以使用fuse方法将任何Future转换为FusedFuture

    // 语法结构
    // select!{
    // <命名 Future 的返回值> = <实现了 FusedFuture 和 Unpin 的 Future 或者 返回 FusedFuture 的表达式> => {
    //    <此分支 Future 执行完成后执行，此处可以使用返回值>
    // }
    // }

    // let future1 = async { 1 };
    // let future2 = async { 2 };

    // 使用fuse方法将Future转换为FusedFuture
    let future1 = async { 1 }.fuse();
    let future2 = async { 2 }.fuse();

    // 使用pin_mut! 在栈上固定
    pin_mut!(future1);
    pin_mut!(future2);

    let r = select! {
       r = future1 => {r},
       r = future2 => {r},
    };
    println!("result: {}", r);

    // 再修改示例，直接传递返回Future的表达式
    let r = select! {
       r = async { 1 }.fuse() => {r},
       r = async { 2 }.fuse() => {r},
    };
    println!("result: {}", r);

    // select!还接受一个 complete 的分支和一个default分支。如果所有Future和Stream都已耗尽
    // （也就是FusedFuture::is_terminated()返回true），则complete将运行。
    // 如果没有 Future 或 Stream 立即准备好，将运行 default
    // 在所有 Future 都已完成的情况下，complete 优先于 default
    let mut pending1 = future::pending::<()>();
    let future1 = async {
        async_std::task::sleep(Duration::from_secs(1)).await;
        2
    }
    .fuse();
    pin_mut!(future1);
    let r = select! {
       _ = pending1 => 1,
       r = future1 => r,
       default => {
          println!("Default!");
          0
       }
    };
    println!("result: {}", r);
}

async fn f_select2() {
    // 然后再看complete分支，你可能会觉得奇怪，select!的分支任意一个准备好，
    // 不就直接执行然后返回了吗，complete压根没机会执行啊
    // 但是select!是不会移动传递的Future的所有权，可以重复使用，
    // 当select!位于loop中时，可能一些Future已经运行过并且完成了
    // 当所有 Future 都完成了，就会运行 complete 分支

    let mut pending1 = future::pending::<()>();
    let future1 = async {
        async_std::task::sleep(Duration::from_secs(1)).await;
        2
    }
    .fuse();
    let future2 = async {
        async_std::task::sleep(Duration::from_secs(1)).await;
        3
    }
    .fuse();

    pin_mut!(future1);
    pin_mut!(future2);

    loop {
        let r = select! {
           _ = pending1 => 1,
           r = future1 => r,
           r = future2 => r,
           complete => {
                 println!("Complete!");
                 break;
           }
        };
        println!("result: {}", r);
    }
}

fn main() {
    // block_on(f_join());
    // block_on(f_join_all());
    // block_on(f_select());
    // block_on(f_try_select());
    // block_on(f_select_all());
    // block_on(f_select_ok());
    // block_on(f_select1());
    block_on(f_select2());
}
