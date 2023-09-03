// Skip策略，忽略型的计时策略，当出现延迟后，仍然所有已经被延迟的计时任务。
// 这种策略总是以定义计时器时的起点为基准，类似等差数量，每一次执行tick的时间点，
// 一定符合 Start + N * Duration

use chrono::Local;
use tokio;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    println!("before: {}", now());

    let mut intv = time::interval_at(
        Instant::now() + Duration::from_secs(5),
        Duration::from_secs(2),
    );
    intv.set_missed_tick_behavior(MissedTickBehavior::Skip);

    time::sleep(Duration::from_secs(10)).await;

    println!("start: {}", now());
    intv.tick().await;
    println!("tick 1: {}", now());
    intv.tick().await;
    println!("tick 2: {}", now());
    intv.tick().await;
    println!("tick 3: {}", now());
}

// 上面通过interval_at()解释清楚了tokio::time::Interval的三种计时策略。
// 但在程序中，更大的可能是使用interval()来定义间隔计时器，它等价于
// interval_at(Instant::now() + Duration)，表示计时起点从现在开始的计时器。

// 此外，可以使用period()方法获取计时器的间隔时长，使用missed_tick_behavior()获取当前的计时策略
