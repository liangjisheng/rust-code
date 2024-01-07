// Instant用于表示时间点，主要用于两个时间点的比较和相关运算
// tokio::time::Instant是对std::time::Instant的封装，添加了一些对齐功能，使其能够适用于tokio runtime。
// Instant是严格单调递增的，绝不会出现时光倒流的现象，即之后的时间点一定晚于之前创建的时间点。
// 但是，tokio time提供了pause()函数可暂停时间点，还提供了advance()函数用于向后跳转到某个时间点。
// tokio::time::Instant::now()用于创建代表此时此刻的时间点。Instant可以直接进行大小比较，还能执行+、-操作

use tokio;
use tokio::time::{self, Duration, Instant};

#[tokio::main]
async fn main() {
    // 创建代表此时此刻的时间点
    let now = Instant::now();

    // Instant 加一个Duration，得到另一个Instant
    let next_3_sec = now + Duration::from_secs(3);
    // Instant之间的大小比较
    println!("{}", now < next_3_sec); // true

    // Instant减Duration，得到另一个Instant
    let new_instant = next_3_sec - Duration::from_secs(2);

    // Instant减另一个Instant，得到Duration
    // 注意，Duration有它的有效范围，因此必须是大的Instant减小的Instant，反之将panic
    let duration = next_3_sec - new_instant;
    println!("duration {:?}", duration);

    // 注意，std::thread::sleep()会阻塞当前线程，而tokio的睡眠不会阻塞当前线程，
    // 实际上tokio的睡眠在进入睡眠后不做任何事，仅仅只是立即放弃CPU，并进入任务轮询队列，
    // 等待睡眠时间终点到了之后被Reactor唤醒，然后进入就绪队列等待被调度
    tokio::time::sleep(Duration::from_millis(500)).await;
    let d = now.elapsed();
    println!("d {:?}", d);

    // 一直睡眠，睡到500毫秒后醒来
    time::sleep_until(Instant::now() + Duration::from_millis(500)).await;

    // 注意，tokio的sleep的睡眠精度是毫秒，因此无法保证也不应睡眠更低精度的时间。
    // 例如不要睡眠100微秒或100纳秒，这时无法保证睡眠的时长。
    // 下面是一个睡眠10微秒的例子，多次执行，会发现基本上都要1毫秒多，去掉执行指令的时间，
    // 实际的睡眠时长大概是1毫秒。另外，将睡眠10微秒改成睡眠100微秒或1纳秒，结果也是接近的

    let start = time::Instant::now();
    // time::sleep(time::Duration::from_nanos(100)).await;
    // time::sleep(time::Duration::from_micros(100)).await;
    time::sleep(time::Duration::from_micros(10)).await;
    println!(
        "sleep {}",
        time::Instant::now().duration_since(start).as_nanos()
    );

    println!("end");
}
