// future_executor.rs

// Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动。其中一个推动它的
// 方式就是在 async 函数中使用 .await 来调用另一个 async 函数，但是这个只能解决
// async 内部的问题，那么这些最外层的 async 函数，谁来推动它们运行呢？答案就是我们
// 之前多次提到的执行器 executor.

// 执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们
// 直到完成。 最开始，执行器会先 poll 一次 Future ，后面就不会主动去 poll 了，
// 而是等待 Future 通过调用 wake 函数来通知它可以继续，它才会继续去 poll 。这种
// wake 通知然后 poll 的方式会不断重复，直到 Future 完成。

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::Context,
        time::Duration,
    },
};

// 引入之前实现的定时器模块
use timer_future::TimerFuture;

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个 Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// 进行中的Future，在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!("first dispatch the future task to executor.");
        self.task_sender.send(task).expect("too many tasks queued.");
    }
}

/// 实现ArcWake，表明怎么去唤醒任务去调度执行。
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    // 实际运行具体的Future任务，不断的接收Future task执行。
    fn run(&self) {
        let mut count = 0;
        while let Ok(task) = self.ready_queue.recv() {
            count = count + 1;
            println!("received task. {}", count);
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                println!("before poll");
                if future.as_mut().poll(context).is_pending() {
                    println!(
                        "executor run the future task, but is not ready, create a future again."
                    );
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                } else {
                    println!("executor run the future task, is ready. the future task is done.");
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 将 TimerFuture 封装成一个任务，分发到调度器去执行
    spawner.spawn(async {
        println!("TimerFuture await");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(3, 0)).await;
        println!("TimerFuture Done");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    executor.run();
}
