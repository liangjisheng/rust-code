// 使用 memmap 创建文件的内存映射，并模拟文件的一些非序列读取。使用内存映射意味着
// 您仅需索引一个切片，而不是使用 seek 方法来导航整个文件。
// Mmap::map 函数假定内存映射后的文件没有被另一个进程同时更改，否则会出现竞态条件

use memmap::Mmap;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    write!(
        File::create("content.txt")?,
        "My hovercraft is full of eels!"
    )?;

    let file = File::open("content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&random_bytes[..], b"My loaf!");

    Ok(())
}
