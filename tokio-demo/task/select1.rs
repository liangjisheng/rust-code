// select!宏的作用是轮询指定的多个异步任务，每个异步任务都是select!的一个分支，当某个分支已完成，
// 则执行该分支对应的代码，同时取消其它分支。简单来说，select!的作用是等待第一个完成的异步任务并
// 执行对应任务完成后的操作

// 每个分支都有一个异步任务，并对异步任务完成后的返回结果进行模式匹配，如果匹配成功，则执行对应的handler

use tokio::{
    self,
    runtime::Runtime,
    time::{self, Duration},
};

async fn sleep(n: u64) -> u64 {
    time::sleep(Duration::from_secs(n)).await;
    n
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 注意，select!本身是【阻塞】的，只有select!执行完，它后面的代码才会继续执行
        tokio::select! {
          v = sleep(2) => println!("sleep 2 secs, branch 1 done: {}", v),
          v = sleep(1) => println!("sleep 1 secs, branch 2 done: {}", v),
        };

        println!("select! done");
    });
}
