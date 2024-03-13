// 当然，读写操作也可以结合ready()来使用，调用ready()时可注册感兴趣的事件，
// 当注册的事件之一发生之后，ready()将返回Ready结构体，Ready结构体有一些
// 布尔判断方法，用来判断某个事件是否发生

use std::io;
use tokio::io::Interest;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    loop {
        // 注册可读和可写事件，并等待事件的发生
        let ready = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await
            .unwrap();

        // 如果注册的事件中，发生了可读事件，则执行如下代码
        if ready.is_readable() {
            let mut data = vec![0; 1024];
            match stream.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return;
                }
            }
        }

        // 如果注册的事件中，发生了可写事件，则执行如下代码
        if ready.is_writable() {
            match stream.try_write(b"hello world") {
                Ok(n) => {
                    println!("write {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    return;
                }
            }
        }
    }
}
