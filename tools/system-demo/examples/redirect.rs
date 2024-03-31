// 生成子进程并将 stdout 和 stderr 重定向到同一个文件。它遵循与运行管道传输的外部
// 命令相同的思想，但是 process::Stdio 会将输出写入指定的文件。对 stdout 和
// stderr 而言，File::try_clone 引用相同的文件句柄。它将确保两个句柄使用相同的光标
// 位置进行写入。
// 下面的实例等同于运行 Unix shell 命令 ls . oops >out.txt 2>&1

use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
