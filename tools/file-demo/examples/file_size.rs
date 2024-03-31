// 通过 WalkDir::min_depth 和 WalkDir::max_depth 方法，可以灵活设置目录的递归深度。
// 下面的实例计算了 3 层子文件夹深度的所有文件的大小总和，计算中忽略根文件夹中的文件

use walkdir::WalkDir;

fn main() {
    let total_size = WalkDir::new(".")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}
