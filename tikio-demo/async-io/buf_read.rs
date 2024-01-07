// 再来介绍AsyncBufReadExt提供的方法的用法，有以下几个方法：
// lines(): 返回Lines，Lines有一个next_line()方法，可不断地从
// BufReader中读取下一行(返回内容不包括换行符)，直到遇到EOF
// read_line(): 从BufReader中读取下一行(返回内容包含换行符)并追加到指定的String buf的尾部
// read_until(): 一直读取，直到遇到指定的字节(分隔符字节)或EOF
// (返回内容包含分隔符)，读取的内容将追加到buf的尾部
// split(): 根据指定的字节将Reader进行划分，返回Split，Split提供了
// next_segment()方法，可不断从BufReader中读取下一个分割片段(返回内容不包含分隔符)直到遇到EOF

// lines()可按行进行异步迭代式的读取

use tokio::{
    self,
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

async fn buf_read() {
    let f = File::open("a.log").await.unwrap();
    let mut lines = BufReader::new(f).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        println!("read line: {}", line);
    }
}

// 类似的，split()是指定分隔符，而不是默认的换行符作为分隔符。例如，指定换行符作为分隔符
// 需注意的是，split()方法只能指定字节作为分隔符，不能指定字符分隔符，另外，Split的
// next_segment() 方法读取的数据会保存到Vec<u8>中，而不是直接返回String
async fn buf_read1() {
    let f = File::open("a.log").await.unwrap();
    let mut lines = BufReader::new(f).split(b'\n');
    while let Some(line) = lines.next_segment().await.unwrap() {
        println!("read line: {}", String::from_utf8(line).unwrap());
    }
}

// read_line()方法则是从缓冲空间中读取一行，读取的内容(包含换行符)会追加到指定的String buf中
async fn buf_read2() {
    let f = File::open("a.log").await.unwrap();
    let mut f = BufReader::new(f);

    let mut data = String::new();
    f.read_line(&mut data).await.unwrap();
    println!("first line: {}", data);
}

// read_until()方法类似于read_line()，只是不是读取到换行符停止，
// 而是读取到指定的分隔符停止。同样的，也只能使用字节分隔符，读取的内容追加到Vec buf中
async fn buf_read3() {
    let f = File::open("a.log").await.unwrap();
    let mut f = BufReader::new(f);

    let mut data = Vec::new();
    f.read_until(b'\n', &mut data).await.unwrap();
    println!("first line: {}", String::from_utf8(data).unwrap());
}

#[tokio::main]
async fn main() {
    // buf_read().await;
    // buf_read1().await;
    // buf_read2().await;
    buf_read3().await;
}
