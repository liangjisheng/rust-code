// tokio::io::stdin(): 得到tokio::io::Stdin，即标准输入Reader，可从标准输入读取数据
// tokio::io::stdout(): 得到tokio::io::Stdout，标准输出Writer，可写向标准输出
// tokio::io::stderr(): 得到tokio::io::Stderr，标准错误Writer，可写向标准错误

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    // 循环从标准输入中读取数据
    loop {
        stdout.write(b"entry somethin: ").await.unwrap();
        stdout.flush().await.unwrap();

        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };

        buf.truncate(n);
        stdout.write(b"data from stdin: ").await.unwrap();
        stdout.write(&buf).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
