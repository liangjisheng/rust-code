// 任何一个异步框架的核心目标都是异步IO，更高效率的IO编程也是多数时候我们使用异步框架的初衷。
// tokio的异步IO组件封装了std::io中的几乎所有东西的异步版本，同步IO和异步IO的API在使用方法上也类似

use tokio::io::{AsyncBufReadExt, AsyncReadExt};

async fn read() {
    let mut f = tokio::fs::File::open("a.log").await.unwrap();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await.unwrap();
    println!("The bytes: {:?}", &buffer[..n]);
}

// 按行读取文件
// 将File转换为BufReader将使得读取更为简便，比如上面可以直接按行读取文件。
// 如果不转换为BufReader，而是直接通过File进行读取，将只能按字节来读取。
// 如果文件中的是字符串数据，那么按字节读取时会比较麻烦
async fn read1() {
    let file = tokio::fs::File::open("a.log").await.unwrap();
    // 将file转换为BufReader
    let mut buf_reader = tokio::io::BufReader::new(file).lines();
    // 每次读取一行
    while let Some(line) = buf_reader.next_line().await.unwrap() {
        // 注意lines()中的行是不带结尾换行符的，因此使用println!()而不是print!()
        println!("{}", line);
    }
}

// 通过read_line()的方式来按行读取
async fn read2() {
    let file = tokio::fs::File::open("a.log").await.unwrap();
    let mut buf_reader = tokio::io::BufReader::new(file);
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

#[tokio::main]
async fn main() {
    // read().await;
    // read1().await;
    read2().await;
}
