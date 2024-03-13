// 需要注意，tokio提供了两种功能的线程：
// 用于异步任务的工作线程(worker thread)
// 用于同步任务的阻塞线程(blocking thread)

// 单个线程或多个线程的runtime，指的都是工作线程，即只用于执行异步任务的线程，
// 这些任务主要是IO密集型的任务。tokio默认会将每一个工作线程均匀地绑定到每一个CPU核心上

// tokio 提供了这两类不同的线程。worker thread只用于执行那些异步任务，异步任务指的是不会阻塞线程的任务
// blocking thread 则用于那些长时间计算的或阻塞整个线程的任务

// blocking thread默认是不存在的，只有在调用了spawn_blocking()时才会创建一个对应的blocking thread

// blocking thread不用于执行异步任务，因此runtime不会去调度管理这类线程，它们在本质上相当于
// 一个独立的thread::spawn()创建的线程，它也不会像block_on()一样会阻塞当前线程。它和独立
// 线程的唯一区别，是blocking thread是在runtime内的，可以在runtime内对它们使用一些异步操作，例如await

use chrono::Local;
use std::thread;
use tokio::{self, runtime::Runtime};

fn now() -> String {
    Local::now().format("%F %T.%3f").to_string()
}

fn main() {
    let rt1 = Runtime::new().unwrap();
    // 创建一个blocking thread，可立即执行(由操作系统调度系统决定何时执行)
    // 注意，不阻塞当前线程
    let task = rt1.spawn_blocking(|| {
        println!("in task: {}", now());
        // 注意，是线程的睡眠，不是tokio的睡眠，因此会阻塞整个线程
        thread::sleep(std::time::Duration::from_secs(1))
    });

    // 小睡1毫秒，让上面的blocking thread先运行起来
    std::thread::sleep(std::time::Duration::from_millis(1));
    println!("not blocking: {}", now());

    // 可在 runtime 内等待 blocking_thread 的完成
    rt1.block_on(async {
        task.await.unwrap();
        println!("after blocking task: {}", now());
    });
}

// 需注意，blocking thread生成的任务虽然绑定了runtime，但是它不是异步任务，
// 不受tokio调度系统控制。因此，如果在block_on()中生成了blocking thread
// 或普通的线程，block_on()不会等待这些线程的完成

// rt.block_on(async{
// 生成一个blocking thread和一个独立的thread
// block on不会阻塞等待两个线程终止，因此block_on在这里会立即返回
// rt.spawn_blocking(|| std::thread::sleep(std::time::Duration::from_secs(10)));
// thread::spawn(||std::thread::sleep(std::time::Duration::from_secs(10)));
// });

// tokio允许的blocking thread队列很长(默认512个)，且可以在runtime build时通过
// max_blocking_threads()配置最大长度。如果超出了最大队列长度，新的任务将放在一个
// 等待队列中进行等待(比如当前已经有512个正在运行的任务，下一个任务将等待，直到有某个
// blocking thread空闲)。

// blocking thread执行完对应任务后，并不会立即释放，而是继续保持活动状态一段时间，
// 此时它们的状态是空闲状态。当空闲时长超出一定时间后(可在 runtime build 时通过
// thread_keep_alive()配置空闲的超时时长)，该空闲线程将被释放。

// blocking thread有时候是非常友好的，它像独立线程一样，但又和runtime绑定，它不受
// tokio的调度系统调度，tokio不会把其它任务放进该线程，也不会把该线程内的任务转移到
// 其它线程。换言之，它有机会完完整整地发挥单个线程的全部能力，而不像worker线程一样，
// 可能会被调度器打断
