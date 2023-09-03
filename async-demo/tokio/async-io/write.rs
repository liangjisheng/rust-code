// AsyncWriteExt提供了以下几个方法：
// write(): 将给定的字节数组中的数据写入到Writer中
// write_all(): 将给定的字节数组中的所有数据写入到Writer中
// write_buf(): 将给定buf的数据写入到Writer，每次写入时，buf会自动维护内部的位移指针
// write_all_buf(): 将给定buf的数据全部写入到Writer
// write_vectored(): 将一个或多个buf的所有数据写入到Writer
// flush(): 将缓冲中的数据刷入目标Writer。适用于BufWriter
// shutdown(): 关闭Writer，关闭时如果(BufWriter的)缓冲中还有数据，则会触发flush保证数据刷入Writer

// 最基础的是write()方法，它尝试将给定的字节数组(即[u8; N])中的
// 所有字节写入到Writer中，但不一定会全部写入成功

use tokio::io::{self, AsyncWriteExt, BufWriter};
use tokio::{self, fs::File};

async fn write1() -> io::Result<()> {
    // 以 write-only 模式打开文件
    // 如果文件不存在，则创建，如果已存在，则截断文件
    let mut f = File::create("a.log").await?;

    let n = f.write(b"hello world").await?;
    println!("write {} bytes", n);
    Ok(())
}

// 和write()类似的是write_all()方法，它要求给定的字节数组的所有数据
// 全部写入成功后才返回，除非遇到错误。
// flush()方法适用于使用了BufWriter的场景。当使用了BufWriter，写入
// 的数据首先写入到一个缓冲空间，在适当的时候(比如缓冲空间已满时)才会将
// 缓冲空间中的数据真正写入到目标，使用flush()可强制将缓冲空间的数据写入到目标
async fn write2() {
    let f = File::create("foo.txt").await.unwrap();
    let mut buffer = BufWriter::new(f);

    // 这次写入只是写入到缓冲空间
    buffer.write_all(b"some bytes").await.unwrap();

    // 将缓冲空间的数据刷入 writer
    buffer.flush().await.unwrap();
}

// shutdown()用于关闭Writer，shutdown之后，无法再通过writer写入新数据。
// 但如果在关闭时，BufWriter的缓冲空间中还有数据，则会自动将数据刷入到writer

// tokio::io::BufStream则同时提供读、写的缓冲功能，它相当于BufReader和
// BufWriter的结合体。也就是说，BufStream的实例即可进行带缓冲的读，也可以进行带缓冲的写
// let f1 = File::open("foo.txt").await.unwrap();
// let mut reader = tokio::io::BufStream::new(f);

#[tokio::main]
async fn main() {
    // let _ = write1().await;
    write2().await;
}
