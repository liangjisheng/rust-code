use futures::executor::{block_on, ThreadPool, ThreadPoolBuilder};
use futures::future::Future;
use futures::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

fn main() {
    const NUM: usize = 3;

    struct Yield {
        rem: usize,
    }

    impl Future for Yield {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.rem == 0 {
                println!("{}:done", thread::current().name().unwrap());
                Poll::Ready(())
            } else {
                println!("self.rem={}", self.rem);
                self.rem -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    for _ in 0..1 {
        let y = Yield { rem: NUM };
        let pool = ThreadPoolBuilder::new()
            .name_prefix("pool-")
            .create()
            .unwrap();

        // 不管子线程执行的任务有没有Ready,不会阻塞主线程
        pool.spawn_ok(y);
    }

    let y = Yield { rem: NUM };
    block_on(y);

    thread::sleep(Duration::from_secs(1));
}

// 从控制台打印中可以看出block_on里的future运行在main线程中
// spawn_ok里的future运行在pool-x(pool-0,pool-1…)线程中
