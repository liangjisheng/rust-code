use std::env;
use std::process;

use minigrep::Config;

// cargo run -- to poem.txt
// IGNORE_CASE=1 cargo run to poem.txt

// 现在看到了屏幕上的错误信息，同时 output.txt 里什么也没有
// 标准输出被重定向到 output.txt 文件中
// cargo run > output.txt

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        // 使用 eprintln! 将错误信息写入标准错误而不是标准输出
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
