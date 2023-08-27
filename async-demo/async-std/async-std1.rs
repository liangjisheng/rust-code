use async_std::{
    fs::File,   // 支持异步操作的文件结构体
    prelude::*, // Future或输入输出流
    task,       // 调用调度器
};

/// 读取一个文件中的64个字节（十进制表示）
async fn file_test() -> Result<(), std::io::Error> {
    let mut file = File::open("test.txt").await?; // ?号表示如果Err，会直接返回
    let mut buffer: [u8; 64] = [0; 64]; // 创建一个64个长度的字节数组
    let n = file.read(&mut buffer).await?; // 每个await都是阻塞
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

fn async_std1() {
    // 阻塞运行 Future
    let res = task::block_on(file_test());
    if res.is_err() {
        println!("error {}", res.unwrap_err())
    }
}

fn main() {
    async_std1();
}


