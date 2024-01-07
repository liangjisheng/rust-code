use std::time::Duration;
use tokio::runtime;
use tokio::task::JoinSet;

fn main() {
    let max_task = 5;
    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(max_task)
        .enable_time()
        .build()
        .unwrap();

    let mut set = JoinSet::new();

    rt.block_on(async {
        println!("tokio_multi_thread");
        for i in 0..20 {
            while set.len() >= max_task {
                if let Some(res) = set.join_next().await {
                    let t = match res {
                        Err(e) => {
                            println!("err {}", e);
                            continue;
                        }
                        Ok(v) => v,
                    };
                    println!("t {}", t);
                };
            }

            set.spawn(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("spawn {}", i);
                i
            });
        }

        // 如果 JoinSet 为空，则 join_next 方法返回 None
        while let Some(res) = set.join_next().await {
            let t = match res {
                Err(e) => {
                    println!("err {}", e);
                    continue;
                }
                Ok(v) => v,
            };
            println!("t {}", t);
        }

        // 下面这种方法也可以等待任务结束
        // while set.len() > 0 {
        //     set.join_next().await;
        // }
    });
}
