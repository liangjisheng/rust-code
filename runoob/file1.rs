use std::fs::OpenOptions;
use std::io::prelude::*;

// 可以使用 OpenOptions 来实现用特定方法打开文件
fn main() -> std::io::Result<()> {
    // let mut file = OpenOptions::new().append(true).open("./test.txt")?;
    // file.write(b" APPEND WORD")?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./test.txt")?;

    file.write(b"COVER")?;

    Ok(())
}
