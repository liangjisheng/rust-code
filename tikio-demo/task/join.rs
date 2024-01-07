// 可以使用await去等待某个异步任务的完成，无论这个异步任务是正常完成还是被取消。
// tokio提供了两个宏tokio::join!和tokio::try_join!。它们可以用于等待多个异步任务全部完成：
// join!必须等待所有任务完成
// try_join!要么等待所有异步任务正常完成，要么等待第一个返回Result Err的任务出现
// 另外，这两个宏都需要Future参数，它们将提供的各参数代表的任务封装成为一个大的task

use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn do_one() {
    println!("doing one: {}", now());
    time::sleep(time::Duration::from_secs(2)).await;
    println!("do one done: {}", now());
}

async fn do_two() {
    println!("doing two: {}", now());
    time::sleep(time::Duration::from_secs(1)).await;
    println!("do two done: {}", now());
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        tokio::join!(do_one(), do_two()); // 等待两个任务均完成，才继续向下执行代码
        println!("all done: {}", now());
    });
}
