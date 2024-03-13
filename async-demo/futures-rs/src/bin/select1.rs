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

// join! 只有等所有 Future 结束后，才能集中处理结果，如果你想同时等待多个 Future
// 且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!:

async fn task_one() { /* ... */
}
async fn task_two() { /* ... */
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }

    // 上面的代码会同时并发地运行 t1 和 t2， 无论两者哪个先完成，都会调用对应的
    // println! 打印相应的输出，然后函数结束且不会等待另一个任务的完成。

    // 但是，在实际项目中，我们往往需要等待多个任务都完成后，再结束，像上面这种其中
    // 一个任务结束就立刻结束的场景着实不多。
}

// 首先，.fuse() 方法可以让 Future 实现 FusedFuture 特征， 而 pin_mut!
// 宏会为 Future 实现 Unpin 特征，这两个特征恰恰是使用 select 所必须的:

// Unpin，由于 select 不会通过拿走所有权的方式使用 Future，而是通过可变引用的
// 方式去使用，这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以
// 继续被其它代码使用。
// FusedFuture 的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对
// 其进行轮询使用。Fuse 意味着熔断，相当于 Future 一旦完成，再次调用 poll 会
// 直接返回 Poll::Pending。

// 只有实现了 FusedFuture，select 才能配合 loop 一起使用。假如没有实现，就算
// 一个 Future 已经完成了，它依然会被 select 不停的轮询执行。

// select! 还支持 default 和 complete 分支:

// complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合 loop 使用，
// loop 用于循环完成所有的 Future
// default 分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行

fn s1() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            // complete => println!("completed!"), // 如果不 break 跳出循环，则会一直打印
            default => panic!(), // 该分支永远不会运行，因为 `Future` 会先运行，然后是 `complete`
        };
    }
    assert_eq!(total, 10);
}

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

// 在 select 循环中并发
// 一个很实用但又鲜为人知的函数是 Fuse::terminated() ，可以使用它构建一个空的 Future
// 空自然没啥用，但是如果它能在后面再被填充呢？
// 考虑以下场景：当你要在 select 循环中运行一个任务，但是该任务却是在 select 循环内部创建时，
// 上面的函数就非常好用了。

async fn get_new_num() -> u8 {
    /* ... */
    5
}

async fn run_on_new_num(_: u8) { /* ... */
}

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // 运行 `run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

// 当某个 Future 有多个拷贝都需要同时运行时，可以使用 FuturesUnordered 类型。
// 下面的例子跟上个例子大体相似，但是它会将 run_on_new_num_fut 的每一个拷贝都
// 运行到完成，而不是像之前那样一旦创建新的就终止旧的。

async fn run_loop1(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                 // 定时器已结束，若 `get_new_num_fut` 没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                 // 收到新的数字 -- 创建一个新的 `run_on_new_num_fut` (并没有像之前的例子那样丢弃掉旧值)
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 运行 `run_on_new_num_futs`, 并检查是否有已经完成的
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

fn main() {
    // block_on(race_tasks());
    s1();
}
