// tokio::task::JoinSet用于收集一系列异步任务，并判断它们是否终止
// 注意，使用JoinSet的spawn()方法创建的异步任务才会被收集

use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    // 创建10个异步任务并收集
    for i in 0..10 {
        // 使用 JoinSet 的 spawn() 方法创建异步任务
        set.spawn(async move { i });
    }

    // join_next() 阻塞直到其中一个任务完成
    set.join_next().await;

    // 使用 JoinSet 的 abort_all() 或直接 Drop JoinSet，都会对所有异步任务进行abort()操作
    set.abort_all();
}

// 如果要等待多个或所有任务完成，则循环join_next()即可。如果JoinSet为空，则该方法返回None
// while let Some(_) = set.join_next().await {}

// 使用JoinSet的shutdown()方法，将先abort_all()，然后join_next()所有任务，直到任务集合为空。
// 使用JoinSet的detach_all()将使得集合中的所有任务都被detach，即时JoinSet被丢弃，被detach的任务也依然会在后台运行
