use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use base64::{decode, encode};
use std::str;

fn main() {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());

    let b64 = general_purpose::STANDARD.encode(b"hello world~");
    println!("b64 {}", b64);

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let b64_url = CUSTOM_ENGINE.encode(b"hello internet~");
    println!("b64_url {}", b64_url);
}
