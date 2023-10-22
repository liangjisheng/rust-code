// 值得关注的generic blanket impls:
// impl<R: Read + ?Sized> Read for &mut R;
// impl<W: Write + ?Sized> Write for &mut W;
// 也就是说，Read类型的任何可变引用也都是Read，Write同理

// 这里我想指出的是，&[u8] 实现了Read，Vec<u8>实现了Write。因此我们可以对我们的文件
// 处理函数进行简单的单元测试，通过使用String转换为&[u8]以及从Vec<u8> 转换为String

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// function we want to test
fn uppercase<R: Read, W: Write>(mut read: R, mut write: W) -> Result<(), io::Error> {
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    let uppercase = buffer.to_uppercase();
    write.write_all(uppercase.as_bytes())?;
    write.flush()?;
    Ok(())
}

// in actual program we'd pass Files
fn example(in_path: &Path, out_path: &Path) -> Result<(), io::Error> {
    let in_file = File::open(in_path)?;
    let out_file = File::open(out_path)?;
    uppercase(in_file, out_file)
}

// however in unit tests we can use Strings!
fn example_test() {
    let in_file: String = "i am screaming".into();
    let mut out_file: Vec<u8> = Vec::new();
    uppercase(in_file.as_bytes(), &mut out_file).unwrap();
    let out_result = String::from_utf8(out_file).unwrap();
    assert_eq!(out_result, "I AM SCREAMING");
}

fn main() {
    example_test();
}
