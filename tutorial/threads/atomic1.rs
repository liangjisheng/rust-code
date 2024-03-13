use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

// Ordering::Relaxed 用于控制原子操作使用的内存顺序

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn a1() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}", Instant::now().sub(s));
}

// 还有一点值得注意: 和Mutex一样，Atomic的值具有内部可变性，你无需将其声明为mut

struct Counter {
    count: u64,
}

fn a2() {
    let n = Mutex::new(Counter { count: 0 });

    n.lock().unwrap().count += 1;

    let n = AtomicU64::new(0);

    n.fetch_add(0, Ordering::Relaxed);
}

fn main() {
    // a1();
    a2();
}
