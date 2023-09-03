// sleep()或sleep_until()都返回time::Sleep类型，它有3个方法可调用：

// deadline(): 返回Instant，表示该睡眠任务的睡眠终点
// is_elapsed(): 可判断此时此刻是否已经超过了该sleep任务的睡眠终点
// reset()：可用于重置睡眠任务。如果睡眠任务未完成，则直接修改睡眠终点，
// 如果睡眠任务已经完成，则再次创建睡眠任务，等待新的终点

// 需要注意的是，reset()要求修改睡眠终点，因此Sleep实例需要是mut的，
// 但这样会消费掉Sleep实例，更友好的方式是使用tokio::pin!(sleep)将sleep给pin在当前栈中，
// 这样就可以调用 as_mut() 方法获取它的可修改版本

use chrono::Local;
use tokio::{self, time};

#[allow(dead_code)]
fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    println!("start: {}", now());
    let slp = time::sleep(time::Duration::from_secs(1));
    tokio::pin!(slp);

    // 重置未完成的睡眠实例
    // 注意调用slp.as_mut().await，而不是slp.await，后者会move消费掉slp
    slp.as_mut()
        .reset(time::Instant::now() + time::Duration::from_secs(2));

    slp.await;
    println!("end: {}", now());

    // 重置已完成的睡眠实例
    println!("start: {}", now());
    let slp = time::sleep(time::Duration::from_secs(1));
    tokio::pin!(slp);

    //注意调用 slp.as_mut().await，而不是 slp.await，后者会 move 消费掉 slp
    slp.as_mut().await;
    println!("end 1: {}", now());

    slp.as_mut()
        .reset(time::Instant::now() + time::Duration::from_secs(2));

    slp.await;
    println!("end 2: {}", now());
}
