use tokio;

// 默认的工作线程数量将和CPU核数(虚拟核，即CPU线程数)相同

fn main() {
    // 创建runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(30));

    // 也可以使用Runtime Builder来配置并创建runtime
    // 创建带有线程池的runtime
    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(8) // 8个工作线程
    //     .enable_io() // 可在runtime中使用异步IO
    //     .enable_time() // 可在runtime中使用异步计时器(timer)
    //     .build() // 创建runtime
    //     .unwrap();
}
