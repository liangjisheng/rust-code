use ansi_term::{Colour, Style};

fn main() {
    // 将彩色文本打印到终端
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );

    // 终端中使用粗体文字
    // 比普通前景色更改更复杂的事情，就是需要构造 Style 结构。用 Style::new() 创建结构，和要设置的属性
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );

    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
