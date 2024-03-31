// 优雅停机信号是一种通知服务器进行优雅停机的机制。在 Unix 系统中，常用的优雅停机信号是
// SIGTERM 和 SIGINT。当收到这些信号时，服务器应该停止接受新的请求，并等待正在处理的请求完成

// https://docs.rs/tokio/latest/tokio/signal/index.html
// https://docs.rs/tokio-signal/latest/tokio_signal/
// https://tokio.rs/tokio/topics/shutdown

use std::time;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};

async fn s1() -> Result<(), Box<dyn std::error::Error>> {
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}

async fn s2() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    // let mut stream = signal(SignalKind::hangup())?;
    // let mut stream = signal(SignalKind::terminate())?;
    let mut stream = signal(SignalKind::interrupt())?;

    // Print whenever a HUP signal is received
    loop {
        stream.recv().await;
        println!("got signal HUP");
        tokio::time::sleep(time::Duration::from_secs(1)).await;
        break;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // s1().await
    s2().await
}
