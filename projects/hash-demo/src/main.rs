use data_encoding::HEXLOWER;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, hmac, rand};
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
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

fn run() -> Result<(), std::io::Error> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXLOWER.encode(digest.as_ref()));

    Ok(())
}

// 使用 ring::hmac 创建一个 hmac::Signature ，然后验证签名是否正确
fn signature() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}

fn main() {
    // _ = run();
    let res = signature();
    if res.is_ok() {
        println!("sign verify ok");
    } else {
        println!("sign verify fail");
    }
}
