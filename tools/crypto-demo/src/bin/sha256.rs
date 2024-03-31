// 先创建文件，写入一些数据。然后使用 digest::Context 计算文件内容的
// SHA-256 摘要 digest::Digest

use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<()> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    let str = String::from("hello, world");
    let digest = sha256_digest(str.as_bytes())?;
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    let mut ctx = Context::new(&SHA256);
    ctx.update(b"hello");
    ctx.update(b", ");
    ctx.update(b"world");
    let multi_part = ctx.finish();
    println!("SHA-256 digest is {}", HEXUPPER.encode(multi_part.as_ref()));

    Ok(())
}
