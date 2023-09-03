// 上面定义的计时器，有几点需要说明清楚：
// 1. interval_at()第一个参数定义的是计时器的开始时间，这样描述不准确，
//    它表述的是最早都要等到这个时间点才开始计时。例如，定义计时器5秒之后开始计时，
//    但在第一次tick()之前，先睡眠了10秒，那么该计时器将在10秒后才开始，
//    但如果第一次tick之前只睡眠了3秒，那么还需再等待2秒该tick计时任务才会完成。
// 2. 定义计时器时，要将其句柄(即计时器变量)声明为mut，因为每次tick时，都需要修改计时器内部的下一次计时起点。
// 3. 不像其它语言中的间隔计时器，tokio的间隔计时器需要手动调用tick()方法来生成临时的异步任务。
// 4. 删除计时器句柄可取消间隔计时器

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

    let start = Instant::now() + Duration::from_secs(5);
    let interval = Duration::from_secs(1);
    let mut intv = time::interval_at(start, interval);

    time::sleep(Duration::from_secs(10)).await;
    intv.tick().await;
    println!("task 1: {}", now()); // 2023-08-28 22:25:50
    intv.tick().await;
    println!("task 2: {}", now()); // 2023-08-28 22:25:50
}

// 注意输出结果中的task 1和task 2的时间点是相同的，说明第一次tick之后，
// 并没有等待1秒之后再执行紧跟着的tick，而是立即执行之。

// 简单解释一下上面示例中的计时器内部的工作流程，假设定义计时器的时间点是19:00:10：

// 定义5秒后开始的计时器intv，该计时器内部有一个字段记录着下一次开始tick的时间点，其值为19:00:15
// 睡眠10秒后，时间点到了19:00:20，此时第一次执行intv.tick()，它将生成一个异步任务，
// 执行器执行时发现此时此刻的时间点已经超过该计时器内部记录的值，于是该异步任务立即完成并进入就绪队列等待调度
// 同时修改计时器内部的值为19:00:16
// 下一次执行tick的时候，此时此刻仍然是19:00:20，已经超过了该计时器内部的19:00:16，因此计时任务立即完成
// 这是tokio Interval在遇到计时延迟时的默认计时策略，叫做Burst。tokio支持三种延迟后的计时策略。
// 可使用set_missed_tick_behavior(MissedTickBehavior)来修改计时策略。

// Burst策略，冲刺型的计时策略，当出现延迟后，将尽量快地完成接下来的tick，直到某个tick赶上它正常的计时时间点
