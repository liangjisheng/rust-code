// return-if-changed 指令告诉 Cargo 只有在脚本内容发生变化时，才能重新编译和运行构建脚本。
// 如果没有这一行，项目的任何文件发生变化都会导致 Cargo 重新编译运行该构建脚本

use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let out_dir = OsString::from_str("./src/hello").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str { \"Hello, World!\" }",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

// use std::env;
// use std::fs;
// use std::path::Path;
//
// fn main() {
//     let out_dir = env::var_os("OUT_DIR").unwrap();
//     let dest_path = Path::new(&out_dir).join("hello.rs");
//     fs::write(
//         &dest_path,
//         "pub fn message() -> &'static str { \"Hello, World!\" }",
//     )
//     .unwrap();
//     println!("cargo:rerun-if-changed=build.rs");
// }
