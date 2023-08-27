use std::fs;
use std::io::prelude::*;

fn main() {
    // read();
    // read1();
    // write();
    write1();
}

fn read() {
    let text = fs::read_to_string("./test.txt").unwrap();
    println!("{}", text);

    // 用 std::fs::read 函数读取 u8 类型集合
    let text1 = fs::read("./test.txt").unwrap();
    println!("{:?}", text1);
}

// std::fs::File 的 open 方法是"只读"打开文件，并且没有配套的 close 方法
// 因为 Rust 编译器可以在文件不再被使用时自动关闭文件
fn read1() {
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("./test.txt").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}

fn write() {
    // 一次性写入请谨慎使用！因为它会直接删除文件内容（无论文件多么大）。如果文件不存在就会创建文件
    fs::write("./test.txt", "FROM RUST PROGRAM").unwrap();
}

// 如果想使用流的方式写入文件内容，可以使用 std::fs::File 的 create 方法
fn write1() {
    let mut file = fs::File::create("./test.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}
