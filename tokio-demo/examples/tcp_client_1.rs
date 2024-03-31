use std::fs::read;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::time::Duration;

async fn write_to_server(writer: OwnedWriteHalf) {
    let mut buf_writer = tokio::io::BufWriter::new(writer);
    let mut i = 0;
    loop {
        let content = format!("hello id {}\n", i);
        print!("content {}", content);
        if let Err(e) = buf_writer.write_all(content.as_bytes()).await {
            eprintln!("write to server failed: {}", e);
        }
        if let Err(e) = buf_writer.flush().await {
            eprintln!("flush to server failed: {}", e);
        }

        i = i + 1;
        if i >= 5 {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn read_from_server(reader: OwnedReadHalf) {
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(e) => {
                eprintln!("read from server error {}", e);
                break;
            }
            // 遇到了EOF
            Ok(0) => {
                println!("server closed");
                break;
            }
            Ok(n) => {
                // read_line() 读取时会包含换行符，因此去除行尾换行符
                // 将 buf.drain(..) 会将buf清空，下一次read_line读取的内容将从头填充而不是追加
                buf.pop();
                let content = buf.drain(..).as_str().to_string();
                println!("read {} bytes from server. content: {}", n, content);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (server_reader, server_writer) = stream.into_split();

    let mut write_task = tokio::spawn(async {
        write_to_server(server_writer).await;
    });

    let mut read_task = tokio::spawn(async {
        read_from_server(server_reader).await;
    });

    // wait task finish.
    if tokio::try_join!(&mut write_task, &mut read_task).is_err() {
        eprintln!("read_task/write_task terminated");
        write_task.abort();
        read_task.abort();
    }
}
