// 递归地查找当前目录中的所有 PNG 文件。在本实例中，** 模式用于匹配当前目录及其所有子目录。
// 在路径任意部分使用 ** 模式，例如，/media/**/*.png 匹配 media 及其子目录中的所有 PNG 文件

use error_chain::error_chain;

use glob::glob;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

fn main() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
