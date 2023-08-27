// https://zhuanlan.zhihu.com/p/606991033

// futures-rs 提供的许多函数、宏，比如常用的join!、select!等

use async_std;
use futures::executor::block_on;
use futures::future::{self, BoxFuture, FusedFuture, FutureExt, TryFutureExt};
use futures::future::{join, join_all, try_join, try_join_all};
use futures::future::{poll_fn, select_ok, try_join3, OptionFuture};
use futures::future::{select, select_all, try_select, Either};
use futures::stream::{self, FuturesUnordered, StreamExt};
use futures::task::{Context, Poll};
use futures::{pin_mut, select};
use std::ops::Index;
use std::time::{self, Duration};

async fn f_lazy() {
    // 创建一个允许延迟执行闭包的新Future。所提供的闭包只在Future被轮询时运行
    let lazy_future = future::lazy(|context| 1);
    assert_eq!(lazy_future.await, 1);
}

async fn f_maybe_done() {
    // maybe_done 包裹传递的 Future，用于表示一个可能已经完成的 Future
    // 通过take_output方法，可以知道Future是否已经完成，没完成返回None，完成了就返回值
    // try_maybe_done是TryFuture版本

    let future = future::maybe_done(async { 5 });
    pin_mut!(future);
    assert_eq!(future.as_mut().take_output(), None);
    let () = future.as_mut().await;
    assert_eq!(future.as_mut().take_output(), Some(5));
    assert_eq!(future.as_mut().take_output(), None);

    let future = future::try_maybe_done(async { Ok::<u32, ()>(5) });
    pin_mut!(future);
    assert_eq!(future.as_mut().take_output(), None);
    assert!(future.as_mut().await.is_ok());
    assert_eq!(future.as_mut().take_output(), Some(5));
    assert_eq!(future.as_mut().take_output(), None);

    let future = future::try_maybe_done(async { Err::<u32, u32>(5) });
    pin_mut!(future);
    assert_eq!(future.as_mut().take_output(), None);
    assert_eq!(future.as_mut().await.unwrap_err(), 5);
    assert_eq!(future.as_mut().take_output(), None);

    // 与 FusedFuture::is_terminated() 有点类似，但是也不同，
    // 使用 maybe_done 包装 Future 不会对 Future 有任何更改
    // fuse 包装的 Future 完成后，将被销毁并且无法被 poll
}

async fn f_poll_fn() {
    // poll_fn 通过闭包快速创建一个 Future
    fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
        Poll::Ready("Hello, World!".into())
    }

    let read_future = poll_fn(read_line);
    assert_eq!(read_future.await, "Hello, World!".to_owned());
}

async fn f_ready() {
    // ready: Creates a future that is immediately ready with a value.
    // ok: 创建一个 future 返回 Result::Ok
    // err 创建一个 future 返回 Result::Err

    let a = future::ready(true);
    assert_eq!(a.await, true);

    let a = future::ok::<bool, bool>(true);
    assert_eq!(a.await, Ok(true));

    let a = future::err::<bool, bool>(true);
    assert_eq!(a.await, Err(true))
}

// pending!宏将返回一次Poll::Pending，再次poll就会准备好
// pending方法则是一直返回Poll::Pending

async fn f_poll_immediate() {
    // 创建一个立刻准备好的Future，poll结果将永远是Pool::Ready，使用Option去表示Future是否真正准备好

    let r = future::poll_immediate(async { 1_u32 });
    assert_eq!(r.await, Some(1));

    let p = future::poll_immediate(future::pending::<i32>());
    assert_eq!(p.await, None);

    let f = async {
        futures::pending!(); // 返回一次 Poll::pending
        42_u8
    };
    pin_mut!(f);
    assert_eq!(None, future::poll_immediate(&mut f).await);
    assert_eq!(42, f.await);
}

async fn f_option_future() {
    // 一个可选的Future
    let mut a: OptionFuture<_> = Some(async { 123 }).into();
    assert_eq!(a.await, Some(123));
    a = None.into();
    assert_eq!(a.await, None);
}

fn main() {
    // block_on(f_lazy());
    // block_on(f_maybe_done());
    // block_on(f_poll_fn());
    // block_on(f_ready());
    // block_on(f_poll_immediate());
    block_on(f_option_future());
}
