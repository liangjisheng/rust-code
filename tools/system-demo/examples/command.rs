// 将 git log --oneline 作为外部命令 Command 运行，并使用 Regex 检查其 Output，
// 以获取最后 5 次提交的哈希值和消息

use error_chain::error_chain;

use regex::Regex;
use std::process::Command;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Regex(regex::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn main() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(
        r"(?x)
                               ([0-9a-fA-F]+) # 提交的哈希值
                               (.*)           # 提交信息",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
