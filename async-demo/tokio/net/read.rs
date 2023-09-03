// 通常，读相关的操作(try_read、peek等)会结合readable()来使用，写相关的操作
// (try_write)会结合writable()来使用。但是注意，即便readable()、writable()
// 的返回分别代表了可读和可写，但这个可读、可写的就绪事件并不能确保真的可读可写，
// 因此读、写时要做好判断

use std::io;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut msg = vec![0; 1024];

    loop {
        // 等待可读事件的发生
        stream.readable().await.unwrap();

        // 即便readable()返回代表可读，但读取时仍然可能返回WouldBlock
        match stream.try_read(&mut msg) {
            Ok(n) => {
                // 成功读取了n个字节的数据
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return;
            }
        }
    }

    println!("GOT = {:?}", msg);
}
