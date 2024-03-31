// 对于通过 PBKDF2 密钥派生函数 pbkdf2::derive 生成的加密（加盐算法）密码，
// 使用 ring::pbkdf2 进行散列（哈希）运算，使用 pbkdf2::verify 验证散列
// （哈希）运算是否正确。salt 值是使用 SecureRandom::fill 生成的，salt
// 字节数组被其安全生成的随机数填充。

use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

fn main() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    // let mut salt = salt("budshome");
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = "Guess Me If You Can，猜猜我是谁";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {:?}", &salt);
    println!("Salt: {}", HEXUPPER.encode(&salt));

    println!("PBKDF2 hash: {:?}", &pbkdf2_hash);
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}

// fn salt(username: &str) -> Vec<u8> {
//     let mut salt = Vec::with_capacity(username.as_bytes().len());
//     salt.extend(username.as_bytes());
//     salt
// }
