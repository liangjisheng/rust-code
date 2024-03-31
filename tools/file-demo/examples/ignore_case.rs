// 在 /media/ 目录中查找与正则表达模式 img_[0-9]*.png 匹配的所有图像文件。

// 一个自定义 MatchOptions 结构体被传递给 glob_with 函数，使全局命令模式下
// 不区分大小写，同时保持其它选项的默认值 Default

use error_chain::error_chain;
use glob::{glob_with, MatchOptions};

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

fn main() -> Result<()> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("/media/img_[0-9]*.png", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}
