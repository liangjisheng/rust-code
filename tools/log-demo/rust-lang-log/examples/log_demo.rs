use log::Level::Debug;
use log::{debug, error, info, log, log_enabled, trace, warn, Level};

struct Connection {
    port: i32,
    speed: f64,
}

fn l1() {
    let conn_info = Connection {
        port: 40,
        speed: 3.20,
    };

    info!(
        "Connected to port {} at {} Mb/s",
        conn_info.port, conn_info.speed
    );
    info!(target: "connection_events", "Successful connection, port: {}, speed: {}",
      conn_info.port, conn_info.speed);
}

struct Data {
    x: i32,
    y: i32,
}

fn expensive_call() -> Data {
    std::thread::sleep(std::time::Duration::from_secs(2));
    Data { x: 1, y: 2 }
}

fn l2() {
    // log 还提供了 log! 和 log_enabled! 宏，后者用于确定一条消息在
    // 当前模块中，对于给定的日志级别是否能够被记录

    // 判断能否记录 Debug 消息
    if log_enabled!(Debug) {
        let data = expensive_call();
        // 下面的日志记录较为昂贵，因此我们先在前面判断了是否能够记录，能，才继续这里的逻辑
        debug!("expensive debug data: {} {}", data.x, data.y);
    }
    if log_enabled!(target: "Global", Debug) {
        let data = expensive_call();
        debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
    }

    let data = (42, "Forty-two");
    let private_data = "private";

    log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
    data.0, data.1, private_data);
}

fn main() {
    // l1();
    l2();
}
