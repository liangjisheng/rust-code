// tokio::time::timeout()或tokio::time::timeout_at()可设置一个异步任务的完成超时时间，
// 前者接收一个Duration和一个Future作为参数，后者接收一个Instant和一个Future作为参数。
// 这两个函数封装异步任务之后，返回time::Timeout，它也是一个Future。
// 如果在指定的超时时间内该异步任务已完成，则返回该异步任务的返回值，如果未完成，则异步任务被撤销并返回Err

// 得到time::Timeout实例res后，可以通过res.get_ref()或者res.get_mut()
// 获得Timeout所封装的Future的可变和不可变引用，使用res.into_inner()获得所封装的Future，
// 这会消费掉该Future。
// 如果要取消Timeout的计时等待，直接删除掉Timeout实例即可

use chrono::Local;
use tokio::{self, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    // 将 1 改为 3 后会返回值
    let res = time::timeout(time::Duration::from_secs(1), async {
        println!("sleeping: {}", now());
        time::sleep(time::Duration::from_secs(2)).await;
        33
    });

    match res.await {
        Err(_) => println!("task timeout: {}", now()),
        Ok(data) => println!("get the res '{}': {}", data, now()),
    };
}
