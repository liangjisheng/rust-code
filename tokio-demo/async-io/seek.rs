// tokio::fs::File已经实现了AsyncSeek，当打开a.log文件时，可设置它的偏移指针。
// 假设a.log文件中存放了 abcdefghij 共10个字节的数据

// std::io::SeekFrom，它是一个Enum，用来描述偏移位置

use std::io::SeekFrom;
use tokio::{
    self,
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};

#[tokio::main]
async fn main() {
    // 只读方式打开文件时，偏移位置offset = 0
    let mut f = File::open("a.log").await.unwrap();

    // seek()设置offset = 4，从offset = 4开始读取，即从第5个字节开始读取
    // seek()返回设置后的偏移位置
    let n = f.seek(SeekFrom::Start(5)).await.unwrap();
    println!("set, offset = {}", n);

    let mut str = String::new();
    f.read_to_string(&mut str).await.unwrap();
    // 返回当前的偏移位置
    let n = f.stream_position().await.unwrap();
    println!("after read, offset = {}, data = {}", n, str);

    // 将偏移指针重置于offset = 0处
    f.rewind().await.unwrap();
    let n = f.stream_position().await.unwrap();
    println!("rewind, offset = {}", n);
}
