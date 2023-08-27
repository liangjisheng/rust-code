use std::io::stdin;

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }

    println!("input a line:");
    let mut str_buf = String::new();

    // std::io::stdin 包含 read_line 读取方法，可以读取一行字符串到缓冲区，
    // 返回值都是 Result 枚举类，用于传递读取中出现的错误，所以常用 expect 或 unwrap 函数来处理错误
    stdin()
        .read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}
