// 任何一个异步框架的核心目标都是异步IO，更高效率的IO编程也是多数时候我们使用异步框架的初衷。
// tokio的异步IO组件封装了std::io中的几乎所有东西的异步版本，同步IO和异步IO的API在使用方法上也类似

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;

async fn read() {
    let mut f = File::open("test.txt").await.unwrap();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await.unwrap();
    println!("The bytes: {:?}", &buffer[..n]);
}

// 按行读取文件
// 将File转换为BufReader将使得读取更为简便，比如上面可以直接按行读取文件。
// 如果不转换为BufReader，而是直接通过File进行读取，将只能按字节来读取。
// 如果文件中的是字符串数据，那么按字节读取时会比较麻烦
async fn buf_read() {
    let file = File::open("test.txt").await.unwrap();
    // 将file转换为BufReader
    let mut buf_reader = tokio::io::BufReader::new(file).lines();
    // 每次读取一行
    while let Some(line) = buf_reader.next_line().await.unwrap() {
        // 注意lines()中的行是不带结尾换行符的，因此使用println!()而不是print!()
        println!("{}", line);
    }
}

// 通过read_line()的方式来按行读取
async fn buf_read1() {
    let file = File::open("test.txt").await.unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();

    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => panic!("read file error"),
            // 遇到了文件结尾，即EOF
            Ok(0) => break,
            Ok(_n) => {
                // read_line()总是保留行尾换行符(如果有的话)，因此使用print!()而不是println!()
                print!("{}", buf);
                // read_line()总是将读取的内容追加到buf，因此每次读取完之后要清空buf
                buf.clear();
            }
        }
    }
}

async fn tcp_read() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buffer = [0; 10];
    let n = stream.read(&mut buffer).await?;
    println!("The bytes read: {:?}", &buffer[..n]);
    Ok(())
}

async fn read_all() -> io::Result<()> {
    let mut file = File::open("test.txt").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    println!("The bytes read: {:?}", buffer);
    Ok(())
}

async fn copy_file() -> io::Result<()> {
    let mut source_file = File::open("test.txt").await?;
    let mut dest_file = File::create("dest.txt").await?;
    let mut buffer = [0; 1024];
    loop {
        let n = source_file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        dest_file.write_all(&buffer[..n]).await?;
    }
    Ok(())
}

async fn copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut reader, &mut writer).await?;

    assert_eq!(&b"hello"[..], &writer[..]);
    Ok(())
}

async fn buf_copy_file() -> io::Result<()> {
    let file = File::open("test.txt").await?;
    let mut reader = BufReader::new(file);
    let mut writer = BufWriter::new(io::stdout());
    let mut line = String::new();
    loop {
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            break;
        }
        writer.write_all(line.as_bytes()).await?;
        line.clear();
    }
    writer.flush().await?;

    Ok(())
}

async fn timeout_read() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buffer = [0; 10];
    let n =
        tokio::time::timeout(std::time::Duration::from_secs(5), stream.read(&mut buffer)).await??;
    println!("The bytes read: {:?}", &buffer[..n]);
    Ok(())
}

#[tokio::main]
async fn main() {
    // read().await;
    // buf_read().await;
    // buf_read1().await;
    // tcp_read().await;
    // let _ = read_all().await;
    // let _ = copy_file().await;
    // let _ = buf_copy_file().await;
    // let _ = timeout_read().await;
}
