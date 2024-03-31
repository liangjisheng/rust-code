// 使用 unicode-segmentation crate 中的 UnicodeSegmentation::graphemes
// 函数，从 UTF-8 字符串中收集个别的 Unicode 字符

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "é");
}
