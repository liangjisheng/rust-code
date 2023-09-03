// Barrier是一种让多个并发任务在某种程度上保持进度同步的手段。
// 例如，一个任务分两步，有很多个这种任务并发执行，但每个任务中的第二步都要求所有任务的第一步已经完成。
// 这时可以在第二步之前使用屏障，这样可以保证所有任务在开始第二步之前的进度是同步的。

// 当然，也不一定要等待所有任务的进度都同步，可以设置等待一部分任务的进度同步。也就是说，
// 让并发任务的进度按批次进行同步。第一批的任务进度都同步后，这一批任务将通过屏障，
// 但是该屏障依然会阻挡下一批任务，直到下一批任务的进度都同步之后才放行

use std::sync::Arc;
use tokio::sync::Barrier;
use tokio::time;

async fn b1() {
    let mut handles = Vec::with_capacity(10);

    // 参数10表示屏障宽度为10，只等待10个任务达到屏障点就放行这一批任务
    // 也就是说，某时刻已经有9个任务在等待，当第10个任务调用wait的时候，屏障将放行这一批
    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let c = barrier.clone();
        handles.push(tokio::spawn(async move {
            println!("before wait");

            // 在此设置屏障，保证10个任务都已输出 before wait 才继续向下执行
            let wait_result = c.wait().await;
            println!("after wait");
            wait_result
        }));
    }

    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    assert_eq!(num_leaders, 1);
}

// Barrier调用wait()方法时，返回BarrierWaitResult，该结构有一个is_leader()方法，
// 可以用来判断某个任务是否是该批次任务中的第一个任务。每一批通过屏障的任务都只有一个leader，
// 其余非leader任务调用is_leader()都将返回false。

// 使用屏障时，一定要保证可以到达屏障点的并发任务数量是屏障宽度的整数倍，
// 否则多出来的任务将一直等待。例如，将屏障的宽度设置为10(即10个任务一批)，
// 但是有15个并发任务，多出来的5个任务无法凑成完整的一批，这5个任务将一直等待

async fn b2() {
    let barrier = Arc::new(Barrier::new(10));

    for i in 1..=15 {
        let b = barrier.clone();
        tokio::spawn(async move {
            println!("data before: {}", i);

            b.wait().await; // 15个任务中，多出5个任务将一直在此等待
            time::sleep(time::Duration::from_millis(10)).await;
            println!("data after: {}", i);
        });
    }
    time::sleep(time::Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() {
    // b1().await;
    b2().await;
}
