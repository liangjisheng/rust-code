use std::panic;

// 抓取 panic, 打印错误信息后，程序继续运行

fn main() {
    let result = panic::catch_unwind(|| {
        println!("Hello");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh, no");
    });

    assert!(result.is_err());

    println!("Hello, end of main()");
    // panic!("Panic self!");
}

// 同时, 也可以在 Cargo.toml 中修改 panic 的行为:
// [profile.dev]
// panic = "unwind"

// [profile.release]
// panic = "abort"
