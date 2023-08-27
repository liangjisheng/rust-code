use std::io::Write;

fn r1() {
    let mut line = String::new();
    println!("请输入你的名字:");
    // 标准库提供的 std::io::stdin() 会返回返回当前进程的标准输入流 stdin 的句柄
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("你好 , {}", line);
    println!("读取的字节数为：{}", b1);
}

fn w1() {
    // 标准库提供的 std::io::stdout() 会返回返回当前进程的标准输出流 stdout 的句柄
    let b1 = std::io::stdout().write("alice ".as_bytes()).unwrap();
    let b2 = std::io::stdout()
        .write(String::from("bob").as_bytes())
        .unwrap();
    std::io::stdout()
        .write(format!("\n写入的字节数为：{}\n", (b1 + b2)).as_bytes())
        .unwrap();
}

// 输出命令行传递的所有参数
fn c1() {
    let params = std::env::args();
    println!("总共有 {} 个命令行参数", params.len());
    for arg in params {
        println!("[{}]", arg);
    }
}

fn cmd_sum() {
    let cmd_line = std::env::args();
    println!("总共有 {} 个命令行参数", cmd_line.len()); // 传递的参数个数

    let mut sum = 0;
    let mut has_read_first_arg = false;

    //迭代所有参数并计算它们的总和
    for arg in cmd_line {
        if has_read_first_arg {
            // 跳过第一个参数，因为它的值是程序名
            let value = arg.parse::<i32>();
            if value.is_err() {
                continue;
            }
            sum += value.unwrap();
        }
        has_read_first_arg = true; // 设置跳过第一个参数，这样接下来的参数都可以用于计算
    }
    println!("和值为: {}", sum);
}

fn main() {
    // r1();
    // w1();
    // c1();
    cmd_sum();
}
