// Notify提供了一种简单的通知唤醒功能，它类似于只有一个信号灯的信号量

use std::sync::Arc;
use tokio::sync::Notify;

async fn n1() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        notify2.notified().await;
        println!("received notification");
    });

    println!("sending notification");
    notify.notify_one();
}

// Notify::new()创建Notify实例，Notify实例初始时没有permit位，permit可认为是执行权。
// 每当调用notified().await时，将判断此时是否有执行权，如果有，则可直接执行，否则将进入等待。
// 因此，初始化之后立即调用notified().await将会等待。
// 每当调用notify_one()时，将产生一个执行权，但多次调用也最多只有一个执行权。
// 因此，调用notify_one()之后再调用notified().await则并无需等待

// Notify还有一个notify_waiters()方法，它不会释放执行权，但是它会一次性唤醒所有正在等待的等候者。
// 严格来说，是让当前已经注册的等候者(即已经调用notified()，但是还未await)在下次等待的时候，可以直接通过
async fn n2() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    // 注册两个等候者
    let notified1 = notify.notified();
    let notified2 = notify.notified();

    let handle = tokio::spawn(async move {
        println!("sending notifications");
        notify2.notify_waiters();
    });

    // 两个等候者的 await 都会直接通过
    notified1.await;
    notified2.await;
    println!("received notifications");
}

#[tokio::main]
async fn main() {
    // n1().await;
    n2().await;
}
