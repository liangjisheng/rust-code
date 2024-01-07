// Delay策略，延迟性的计时策略，当出现延迟后，仍然按部就班地每隔指定的时长计时。
// 在内部，这种策略是在每次执行tick之后，都修改下一次计时起点为 Instant::now() + Duration
// 因此，这种策略下的任何相邻两次的tick，其中间间隔的时长都至少达到 Duration

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
    intv.set_missed_tick_behavior(MissedTickBehavior::Delay);

    time::sleep(Duration::from_secs(10)).await;

    println!("start: {}", now());
    intv.tick().await;
    println!("tick 1: {}", now());
    intv.tick().await;
    println!("tick 2: {}", now());
    intv.tick().await;
    println!("tick 3: {}", now());
}
