// 递归下行到子目录的过程中，使用 filter_entry 对目录中的条目传递 is_not_hidden 断言，
// 从而跳过隐藏的文件和目录。Iterator::filter 可应用到要检索的任何目录 WalkDir::DirEntry，
// 即使父目录是隐藏目录。

// 根目录 "." 的检索结果，通过在断言 is_not_hidden 中使用 WalkDir::depth 参数生成

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}
