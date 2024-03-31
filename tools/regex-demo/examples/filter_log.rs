// 读取名为 application.log 的文件，并且只输出包含下列内容的行：“version X.X.X”、
// 端口为 443 的 IP 地址（如 “192.168.0.1:443”）、特定警告。

// 正则表达集构造器 regex::RegexSetBuilder 构建了正则表达式集 regex::RegexSet
// 由于反斜杠在正则表达式中非常常见，因此使用原始字符串字面量可以使它们更具可读性

use error_chain::error_chain;

use regex::RegexSetBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Regex(regex::Error);
    }
}

fn main() -> Result<()> {
    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}
