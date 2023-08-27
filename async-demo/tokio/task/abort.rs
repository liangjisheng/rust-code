// 正在执行的异步任务可以随时被abort()取消，取消之后的任务返回JoinError类型

use tokio::{self, runtime::Runtime, time};

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let task = tokio::task::spawn(async {
            time::sleep(time::Duration::from_secs(10)).await;
        });

        // 让上面的异步任务跑起来
        time::sleep(time::Duration::from_millis(1)).await;
        // 取消任务
        task.abort();

        // 取消任务之后，可以取得JoinError
        let abort_err = task.await.unwrap_err();
        println!("{}", abort_err.is_cancelled());
        println!("{}", abort_err.to_string());
    })
}

// 如果异步任务已经完成，再对该任务执行abort()操作将没有任何效果。
// 也就是说，没有JoinError，task.await.unwrap_err()将报错，而task.await.unwrap()则正常
