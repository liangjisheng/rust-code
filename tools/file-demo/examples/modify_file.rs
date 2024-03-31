// 通过调用 env::current_dir 获取当前工作目录，然后通过 fs::read_dir 读取目录中的每个条目，
// 通过 DirEntry::path 提取条目路径，以及通过通过 fs::Metadata 获取条目元数据
// Metadata::modified 返回条目自上次更改以来的运行时间 SystemTime::elapsed
// Duration::as_secs 将时间转换为秒，并与 24 小时（24 * 60 * 60 秒）进行比较
// Metadata::is_file 用于筛选出目录

use error_chain::error_chain;

use std::{env, fs};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
    }
}

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}
