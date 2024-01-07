// peek()可读取TcpStream中的数据，但是和其它读取操作不同，peek()读取之后不会消费TcpStream中的数据

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut b1 = [0; 10];
    let mut b2 = [0; 10];

    let n = stream.peek(&mut b1).await.unwrap();
    let n1 = stream.read(&mut b2[..n]).await.unwrap();
}
