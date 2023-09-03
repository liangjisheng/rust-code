// tokio::io::duplex()提供了类似套接字的全双工读写管道
// DuplexStream可读也可写，当管道为空时，读操作会进入等待，
// 当管道空间已满时，写操作会进入等待

// 在两端通信过程中，任意一端的关闭，都会导致写操作报错Err(BrokenPipe)，
// 但读操作会继续读取直到管道的内容被读完遇到EOF。
// DuplexStream实现了Send和Sync，因此可以跨线程、跨任务进行通信

use chrono::Local;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio::{self, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn write_duplex(r: &mut DuplexStream) -> io::Result<usize> {
    r.write(now().as_bytes()).await
}

async fn read_duplex(mut r: DuplexStream) {
    let mut buf = [0u8; 1024];
    loop {
        match r.read(&mut buf).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if let Ok(data) = std::str::from_utf8(&buf[..n]) {
                    println!("read from duplex: {}", data);
                }
            }
        };
    }
}

async fn read_duplex1(r: DuplexStream) {
    // 将DuplexStream分离为Reader和Writer，
    // 不使用Writer，因此关闭Writer
    let (mut reader, writer) = tokio::io::split(r);
    drop(writer);

    let mut buf = [0u8; 1024];
    loop {
        match reader.read(&mut buf).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if let Ok(data) = std::str::from_utf8(&buf[..n]) {
                    println!("read from duplex: {}", data);
                }
            }
        };
    }
}

#[tokio::main]
async fn main() {
    // 参数指定管道的容量
    let (client, mut server) = tokio::io::duplex(64);

    // client read data from server
    tokio::spawn(async move {
        read_duplex1(client).await;
    });

    // server write now() to client
    loop {
        match write_duplex(&mut server).await {
            Err(_) | Ok(0) => break,
            _ => (),
        }
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
