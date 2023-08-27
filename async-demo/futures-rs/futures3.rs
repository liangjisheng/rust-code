// https://zhuanlan.zhihu.com/p/606989449
// futures-rs 通过FutureExt对Future trait 进行了扩展，添加了许多方法

use async_std;
use futures::executor::block_on;
use futures::future::{self, BoxFuture, FusedFuture, FutureExt, TryFutureExt};
use futures::pin_mut;
use futures::stream::{self, StreamExt};
use std::time::{self};

async fn af1() {
    // boxed包装Future到BoxFuture，boxed_local包装Future到LocalBoxFuture
    let future: BoxFuture<i32> = async { 1 }.boxed();
    // let res = future.await;
    // println!("res {}", res);
    assert_eq!(future.await, 1);

    let local_future: BoxFuture<i32> = async { 2 }.boxed();
    let res = local_future.await;
    assert_eq!(res, 2);

    // 映射值，类似于 Option::map 或 Iterator::map
    let future = async { 1 };
    // 映射为了 bool 类型
    let future = future.map(|n| if n == 0 { false } else { true });
    // let res: bool = future.await;
    // println!("res {}", res);
    assert_eq!(future.await, true);

    // map_into 等同于 map(Into::into)
    let future = async { "FS" };
    let value: String = future.map_into().await; // &str 映射为了 String
    assert_eq!(value, "FS".to_string());

    // then 类似于map，不同的是then接受的闭包参数返回值需要是Future
    // future 先执行，将返回值传入闭包执行
    let future = async {
        // println!("return 1");
        1
    };
    let future2 = async {
        // println!("return 2");
        2
    };
    let r = future.then(|r| async move { r + future2.await }).await;
    assert_eq!(r, 3);

    // left_future、right_future 返回的Either<A,B>类型表示两种不同Future的枚举
    // 当要返回两个不同的Future类型时，left_future、right_future方法很有用
    let condition = true;

    // 每个 async 块生成的是不同的实现了Future的类型
    // 所以下面编译失败
    // let future = if condition {
    //     async { true }
    // } else {
    //     async { false }
    // };
    // let x = future.await;

    // 使用left_future、right_future后，都是Either<A,B>结构类型
    // 返回的Either<A,B>实现了Future，可以直接await
    let future = if condition {
        async { true }.left_future()
    } else {
        async { false }.right_future()
    };
    let x = future.await;
    assert_eq!(x, true);

    // into_stream 转换Future为包含单个元素的Stream
    let future = async { 17 };
    let stream = future.into_stream();
    let collected: Vec<_> = stream.collect().await;
    assert_eq!(collected, vec![17]);

    // flatten 扁平化 Future 等同于调用 then(|x| x)
    let nested_future = async {
        // println!("outer");
        async {
            // println!("inner");
            1
        }
    };
    let future = nested_future.flatten();
    assert_eq!(future.await, 1);

    // flatten_stream 扁平化返回Stream的Future，可一次性收集完整个Stream
    let stream = async { stream::iter(vec![1, 2, 3]) };
    let list: Vec<_> = stream.flatten_stream().collect().await;
    assert_eq!(list, vec![1, 2, 3]);
}

// FusedFuture是Future的子trait，如果Future不能再进行poll，那么is_terminated应该返回true
// FusedStream则是FusedFuture的Stream版本
// 如果Future变得不活动并且无法再取得进展，那么应该忽略或丢弃而不是再次poll，则is_terminated也可能返回true
// 重复poll可能导致panic，有了FusedFuture，我们就知道一个Future是否已经结束，就可以避免再次poll
// 一般不允许去poll一个已经结束的Future，直接await会消费自身，你就无法继续await
// 但是可以用Pin包裹，再使用它的引用去await

async fn fuse_demo() {
    let ready = futures::future::ready(true);
    pin_mut!(ready);
    let r = (&mut ready).await;
    assert_eq!(r, true);
    // (&mut ready).await; // 发生 panic!!!

    // 通过 is_terminated 来避免 panic
    let ready = futures::future::ready(true);
    pin_mut!(ready);
    if !ready.is_terminated() {
        let r = (&mut ready).await;
        assert_eq!(r, true);
    }

    // 不会走进来,因为已经 .await 了一次
    if !ready.is_terminated() {
        println!("don't output");
        let r = (&mut ready).await;
        assert_eq!(r, true);
    }

    // fuse包装Future到Fuse<F>结构，包装后，Future poll完成后无法再对它进行poll
    // Fuse<F>实现了FusedFuture，fuse是一种将任何Future转换为FusedFuture的方法
    // Fuse<F>内部的Future在poll完成的时候就会销毁
    // 如果包装完后的 Fuse<F> 在返回 Poll::Ready 后再被 poll，它将一直返回 Poll::Pending
    let ready = futures::future::ready(1);
    let fuse = ready.fuse();
    pin_mut!(fuse);
    let a = (&mut fuse).await;
    assert_eq!(a, 1);

    // 因为一直返回 Poll::Pending，所以一直会堵塞
    // let a = (&mut fuse).await;
    // unreachable!();
}

async fn af2() {
    // 在Future的值输出前，做一些其他事
    let future = async { 1 }.inspect(|&x| println!("value: {}", x));
    assert_eq!(future.await, 1);

    // catch_unwind 捕获 Future 发出的 panic
    let future = future::ready(2);
    assert_eq!(future.catch_unwind().await.unwrap_or(0), 2);

    let future = async { unimplemented!() };
    assert!(future.catch_unwind().await.is_err());
    // 上面捕捉到 panic, 所以 end 会打印出来
    println!("end");
}

async fn af3() {
    // share 为这个Future 创建一个可克隆句柄，其中所有句柄将解析为相同的结果
    // share提供了一种将任意Future转换为可克隆Future的方法，这允许多个线程 poll Future
    let future = futures::future::ready(1).inspect(|e| println!("Finished!"));
    let shared1 = future.shared();
    let shared2 = shared1.clone();

    assert_eq!(1, shared1.await);
    assert_eq!(1, shared2.await);
    // 将只打印一次 Finished!

    // remote_handle 将这个Future转换为在完成时产生()的Future，并将其输出发送到另一个Future
    let future = async {
        println!("Called!");
        1
    };
    let (remote, remote_handler) = future.remote_handle();
    remote.await;
    async_std::task::sleep(time::Duration::from_millis(100)).await;
    let i = remote_handler.await;
    println!("{}", i);

    // unit_error将Future<Output = T>转换为TryFuture<Ok = T, Error =()>
    // never_error将Future<Output = T>转换为TryFuture<Ok = T, Error = Never>

    // now_or_never 执行Future，如果Future第一次调用poll后准备就绪，则返回结果输出。
    // 如果poll返回Poll::Pending，则返回None
    // 这种方法在即时性比等待结果更重要的情况下非常有用。它还可以方便地快速获得已知总是立即解析的未来值
    let future_ready = future::ready("foobar");
    let future_pending = future::pending::<&'static str>();
    assert_eq!(future_ready.now_or_never(), Some("foobar"));
    assert_eq!(future_pending.now_or_never(), None);

    // map_ok映射成功值，map_err映射错误值
    // 映射成功值
    let r = async { Ok::<u32, u32>(1) }.map_ok(|n| n * 10).await;
    assert_eq!(r, Ok(10));
    // map_ok 对错误值，没有效果
    let r = async { Err::<u32, u32>(1) }.map_ok(|n| n * 10).await;
    assert_eq!(r, Err(1));

    // 映射错误值
    let r = async { Err::<u32, u32>(1) }.map_err(|n| n * 10).await;
    assert_eq!(r, Err(10));
    // map_err 对成功值，没有效果
    let r = async { Ok::<u32, u32>(1) }.map_err(|n| n * 10).await;
    assert_eq!(r, Ok(1));

    // map_ok_or_else 将成功值与错误值映射为同一类型
    // 如下示例，将成功值u8类型与错误值&'static str类型映射为bool类型
    let condition = true;
    let result = async {
        if condition {
            Ok::<u8, &'static str>(1)
        } else {
            Err::<u8, &'static str>("false")
        }
    }
    .map_ok_or_else(
        |err| if err == "true" { true } else { false },
        |ok| if ok == 0 { false } else { true },
    )
    .await;
    assert_eq!(result, true);

    // unwrap_or_else 类似于map_ok_or_else，但是不映射成功值，而是将错误值类型映射到成功值类型

    // ok_into 等同于 map_ok(Into::into) err_into 等同于map_err(Into::into)
    let result: String = async { Ok::<&'static str, ()>("Ok") }
        .ok_into()
        .await
        .unwrap();
    assert_eq!(result, "Ok".to_string());

    // `ok_into`等同于`map_ok(Into::into)`
    let result: String = async { Ok::<&'static str, ()>("Ok") }
        .map_ok(Into::into)
        .await
        .unwrap();
    assert_eq!(result, "Ok".to_string());

    let result: String = async { Err::<(), &'static str>("Ok") }
        .err_into()
        .await
        .unwrap_err();
    assert_eq!(result, "Ok".to_string());

    // `err_into`等同于`map_err(Into::into)`
    let result: String = async { Err::<(), &'static str>("Ok") }
        .map_err(Into::into)
        .await
        .unwrap_err();
    assert_eq!(result, "Ok".to_string());

    // and_then、or_else
    // and_then类似于ok_map，or_else类似于err_map，但是接受的不是闭包而是Future

    // inspect_ok、inspect_err
    // 用途与inspect一样，但是inspect_ok在成功时做一些其他事，inspect_err在失败时做一些其他事

    // try_flatten、try_flatten_stream
    // try_flatten、try_flatten_stream 类似于flatten、flatten_stream，区别是它们作用于成功值
}

fn main() {
    // block_on(af1());
    // block_on(fuse_demo());
    // block_on(af2());
    block_on(af3());
}
