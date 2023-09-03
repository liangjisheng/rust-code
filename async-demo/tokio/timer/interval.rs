// tokio::time::interval()和tokio::time::interval_at()用于设置间隔性的任务。
// interval_at(): 接收一个Instant参数和一个Duration参数，
// Instant参数表示间隔计时器的开始计时点，Duration参数表示间隔的时长
// interval(): 接收一个Duration参数，它在第一次被调用的时候立即开始计时

// 注意，这两个函数只是定义了间隔计时器的起始计时点和间隔的时长，要真正开始让它开始计时，
// 还需要调用它的tick()方法生成一个Future任务，并调用await来执行并等待该任务的完成

use chrono::Local;
use tokio::{
    self,
    time::{self, Duration, Instant},
};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    println!("before: {}", now());

    // 计时器的起始计时点：此时此刻之后的5秒后
    let start = Instant::now() + Duration::from_secs(5);
    let interval = Duration::from_secs(1);
    let mut intv = time::interval_at(start, interval);

    // 该计时任务"阻塞"，直到5秒后被唤醒
    intv.tick().await;
    println!("task 1: {}", now());

    // 该计时任务"阻塞"，直到1秒后被唤醒
    intv.tick().await;
    println!("task 2: {}", now());

    // 该计时任务"阻塞"，直到1秒后被唤醒
    intv.tick().await;
    println!("task 3: {}", now());
}
