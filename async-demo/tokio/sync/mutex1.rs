// 什么情况下可以选择使用tokio的Mutex？当跨await的时候，可以考虑使用tokio Mutex
// 因为这时使用标准库的Mutex将编译错误。当然，也有相应的解决方案。

// 什么是跨await？每个await都代表一个异步任务，跨await即表示该异步任务中出现了至少一个子任务。
// 而每个异步任务都可能会被tokio内部偷到不同的线程上执行，因此跨await时要求其父任务实现
// Send Trait，这是因为子任务中可能会引用父任务中的数据

use std::sync::Arc;
use tokio::{
    self,
    time::{self, Duration},
};

async fn add_1(mutex: &std::sync::Mutex<u64>) {
    let mut lock = mutex.lock().unwrap();
    *lock += 1;

    // 子任务，跨await，且引用了父任务中的数据
    // time::sleep(Duration::from_millis(*lock)).await;
}

// std::sync::MutexGuard未实现Send，因此父任务async move{}语句块是非Send的，
// 于是编译报错。但如果上面的示例中没有子任务sleep().await子任务，则编译无错，
// 因为已经可以明确知道该Mutex所在的任务是在当前线程执行的

async fn std_mutex() {
    let mutex = Arc::new(std::sync::Mutex::new(0));

    for i in 0..10 {
        let lock = mutex.clone();
        // 如果把 add_1 中的 time:sleep 注释打开的话会报下面的错误
        // future cannot be sent between threads safely
        // future created by async block is not `Send`
        tokio::spawn(async move {
            add_1(&lock).await;
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}

//对于上面的错误，可简单地使用 tokio::sync::Mutex 来修复

async fn add_2(mutex: &tokio::sync::Mutex<u64>) {
    let mut lock = mutex.lock().await;
    *lock += 1;
    time::sleep(Duration::from_millis(*lock)).await;
}

async fn tokio_mutex() {
    let mutex = Arc::new(tokio::sync::Mutex::new(0));
    for i in 0..10 {
        let lock = mutex.clone();
        tokio::spawn(async move {
            add_2(&lock).await;
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}

// 对于上面的需求，仍然可以继续使用标准库的Mutex，但需要做一些调整。
// 例如，可以在子任务await之前，把所有未实现Send的数据都drop掉，保证子任务无法引用父任务中的任何非Send数据
async fn add_3(mutex: &std::sync::Mutex<u64>) {
    {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    }
    // 子任务，跨await，不引用父任务中的数据
    time::sleep(Duration::from_millis(10)).await;
}

// 这种方案的主要思想是让子任务和父任务不要出现不安全的数据交叉。如果可以的话，
// 应尽量隔离子任务和非Send数据所在的任务。上面的例子已经实现了这一点，
// 但更好的方式是将子任务 sleep().await 从这个函数中移走

async fn add_4(mutex: &std::sync::Mutex<u64>) -> u64 {
    let mut lock = mutex.lock().unwrap();
    *lock += 1;
    *lock
} // 申请的互斥锁在此被释放

async fn std_mutex4() {
    let mutex = Arc::new(std::sync::Mutex::new(0));

    for i in 0..100 {
        let lock = mutex.clone();
        tokio::spawn(async move {
            let n = add_4(&lock).await;
            time::sleep(Duration::from_millis(n)).await;
        });
    }

    time::sleep(Duration::from_secs(1)).await;
    println!("data: {}", mutex.lock().unwrap());
}

#[tokio::main]
async fn main() {
    // std_mutex().await;
    // tokio_mutex().await;
    std_mutex4().await;
}
