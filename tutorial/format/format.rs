// 打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：
// format!：将格式化文本写到字符串。
// print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
// println!: 与 print! 类似，但输出结果追加一个换行符。
// eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
// eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
// 这些宏都以相同的做法解析文本。有个额外优点是格式化的正确性会在编译时检查。

// std::fmt 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：
// fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
// fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本

// Rust 使用 {} 来作为格式化输出占位符，其它语言可能使用的是 %s，%d，%p 等，
// 由于 println! 会自动推导出具体的类型，因此无需手动指定

fn f1() {
    let s1 = format!("test");
    let s2 = format!("hello {}", "world!");
    let s3 = format!("x = {}, y = {y}", 10, y = 30);
    let (x, y) = (1, 2);
    let s4 = format!("{x} + {y} = 3");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}

fn f2() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{:?}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

fn main() {
    // f1()
    f2();
}
