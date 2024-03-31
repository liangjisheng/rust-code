// 使用 ring::hmac 创建字符串的签名 hmac::Signature，然后验证签名是否正确

use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{hmac, rand};

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    println!("{}", &message);

    let signature = hmac::sign(&key, message.as_bytes());
    println!("{:?}", &signature);

    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
