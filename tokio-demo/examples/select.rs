// select!宏的作用是轮询指定的多个异步任务，每个异步任务都是select!的一个分支，当某个分支已完成，
// 则执行该分支对应的代码，同时取消其它分支。简单来说，select!的作用是等待第一个完成的异步任务并
// 执行对应任务完成后的操作

// 每个分支都有一个异步任务，并对异步任务完成后的返回结果进行模式匹配，如果匹配成功，则执行对应的handler

use tokio::{
    self,
    sync::mpsc,
    time::{self, Duration},
};

async fn sleep(n: u64) -> u64 {
    time::sleep(Duration::from_secs(n)).await;
    n
}

async fn demo1() {
    // 注意，select!本身是【阻塞】的，只有select!执行完，它后面的代码才会继续执行
    tokio::select! {
      v = sleep(2) => println!("sleep 2 secs, branch 1 done: {}", v),
      v = sleep(1) => println!("sleep 1 secs, branch 2 done: {}", v),
    };

    println!("select! done");
}

async fn demo2() {
    let (mut tx1, mut rx1) = mpsc::channel(32);
    let (mut tx2, mut rx2) = mpsc::channel(32);

    tokio::spawn(async move {
        tx1.send("hello".to_string()).await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("world".to_string()).await.unwrap();
    });

    loop {
        tokio::select! {
            Some(msg) = rx1.recv() => println!("{}", msg),
            Some(msg) = rx2.recv() => println!("{}", msg),
            else => break,
        }
    }
}

async fn demo3() {
    let mut count = 0u8;
    loop {
        tokio::select! {
            // 如果取消biased，挑选的任务顺序将随机，可能会导致分支中的断言失败
            biased;
            _ = async {}, if count < 1 => { count += 1; assert_eq!(count, 1); }
            _ = async {}, if count < 2 => { count += 1; assert_eq!(count, 2); }
            _ = async {}, if count < 3 => { count += 1; assert_eq!(count, 3); }
            _ = async {}, if count < 4 => { count += 1; assert_eq!(count, 4); }
            else => { break; }
        };
    }
}

#[tokio::main]
async fn main() {
    // demo1().await;
    demo2().await;
}
