use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

fn w1() {
    // 静态方法 create() 用于创建一个文件并返回创建的文件句柄
    // 如果文件已经存在，则会内部调用 open() 打开文件。如果创建失败，比如目录不可写，则会抛出错误
    let mut file = File::create("data.txt").expect("create failed");
    println!("file create ok: {:?}", file);

    // write_all() 用于向当前流写入 buf 中的内容。如果写入成功则返回写入的字节数，如果写入失败则抛出错误
    file.write_all("alice\n".as_bytes()).expect("write failed");
    file.write_all("bob\n".as_bytes()).expect("write failed");
    println!("data written to file");
}

fn r1() {
    // 静态方法 open() 函数用于以只读模式打开一个已经存在的文件，如果文件不存在，
    // 则会抛出一个错误。如果文件不可读，那么也会抛出一个错误
    // let file = File::open("data.txt").unwrap();
    // println!("文件打开成功：{:?}", file);

    // read_to_string() 函数用于读取文件中的所有内容并追加到 buf 中，如果读取成功则返回读取的字节数，如果读取失败则抛出错误
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn append() {
    // Rust 核心和标准库并没有提供直接的函数用于追加内容到文件的末尾
    // 但提供了函数 append() 用于将文件的打开模式设置为 追加
    // 当文件的模式设置为 追加 之后，写入文件的内容就不会代替原先的旧内容而是放在旧内容的后面
    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");
    file.write_all("john\n".as_bytes()).expect("write failed");
    file.write_all("mike\n".as_bytes()).expect("write failed");
    file.write_all("howard\n".as_bytes()).expect("write failed");
    println!("append data ok");
}

fn remove() {
    // 注意，删除可能会失败，即使返回结果为 OK，也有可能不会立即就删除
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}

fn copy() {
    let mut command_line: std::env::Args = std::env::args();
    // 跳过程序名
    command_line.next().unwrap();

    // 原文件
    let source = command_line.next().unwrap();
    // 新文件
    let destination = command_line.next().unwrap();

    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];
    loop {
        let n = file_in.read(&mut buffer).unwrap();
        file_out.write(&buffer[..n]).unwrap();
        if n < buffer.len() {
            break;
        }
    }
    println!("copy ok");
}

fn main() {
    // w1();
    // r1();
    // append();
    // remove();
    copy();
}
