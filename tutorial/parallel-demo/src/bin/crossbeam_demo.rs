use crossbeam_channel::{bounded, unbounded};
use error_chain::error_chain;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;
use std::time::{self, Duration};

fn main() {
    // max_demo();
    // channel_demo();
    // channel_demo1();
    let _ = mutex_demo();
}

fn max_demo() {
    let arr = &[1, 25, -4, 10];

    // 串行计算最大值
    let max = arr.iter().max().unwrap();
    println!("max {}", max);

    // 并行计算最大值
    let max = find_max(arr);
    println!("max {}", max.unwrap());
    assert_eq!(max, Some(25));
}

// 本实例将数组一分为二，并在不同的线程中并行计算
// Scope::spawn 生成一个新的作用域线程
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

fn channel_demo() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // 生产者线程
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // 关闭信道 —— 这是退出的必要条件
            // for 循环在工作线程中
            drop(snd1);
        });

        // 由 2 个线程并行处理
        for _ in 0..n_workers {
            // 从数据源发送数据到接收器，接收器接收数据
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // 在不同的线程中衍生工人
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // 接收数据，直到信道关闭前
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }

        // 关闭信道，否则接收器不会关闭
        // 退出 for 循坏
        drop(snd2);

        // 接收器
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

fn channel_demo1() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}

error_chain! {}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn mutex_demo() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
