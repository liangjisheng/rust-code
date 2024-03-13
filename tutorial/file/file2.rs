use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::str;

// 将整个文件读入到字符串
fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let string_content = fs::read_to_string(path)?;
    Ok(string_content)
}

fn f1() {
    let res = read_file_content_as_string("data.txt").unwrap();
    println!("{}", res)
}

fn read_file_as_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    Ok(byte_content)
}

// 将字节向量转换为 String
fn read_file_as_bytes1(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    let string_content = str::from_utf8(&byte_content)?;

    Ok(string_content.to_string())
}

fn read_file_line_by_line(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            // line是字符串
            Ok(line) => println!("{}", line),
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}

// 以单个字节逐步读取文件
fn read_file_as_single_bytes(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for byte in reader.bytes() {
        match byte {
            // byte 正好是一个字节
            Ok(byte) => println!("{}", byte),
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}

// 以字节块读取文件

const BUFFER_SIZE: usize = 512;

fn read_file_in_byte_chunks(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    loop {
        let buffer = reader.fill_buf()?;
        let buffer_length = buffer.len();
        if buffer_length == 0 {
            break;
        }

        // do_something_with(buffer);

        // 从缓冲区中消耗所有字节
        reader.consume(buffer_length);
    }

    Ok(())
}

fn main() {
    f1();
}
