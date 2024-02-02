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
    println!("{:04}", 123); // => "0123" with leading zeros
}

fn f2() {
    // 通常情况下，`{}` 会被任意变量内容所替换。变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 需要注意的是：带名称的参数必须放在不带名称参数的后面，例如下面代码将报错
    // println!("{abc} {1}", abc = "def", 2);

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

fn f3() {
    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);

    // 上面代码只输出小数点后两位。同时我们还展示了 {} 和 {:?} 的用法，后面如无
    // 特殊区别，就只针对 {} 提供格式化参数说明。

    // 字符串格式化默认使用空格进行填充，并且进行左对齐。
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------
    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
    println!();

    // 数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
    println!();
}

fn f4() {
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");
    println!();

    // 精度可以用于控制浮点数的精度或者字符串的长度
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是alice";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
    println!();

    // 可以使用 # 号来控制数字的进制输出：
    // #b, 二进制
    // #o, 八进制
    // #x, 小写十六进制
    // #X, 大写十六进制
    // x, 不带前缀的小写十六进制
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);
    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);
    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
    println!();

    // 指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
}

fn get_person() -> String {
    String::from("alice")
}

fn f5() {
    // 指针地址
    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // 有时需要输出 {和}，但这两个字符是特殊字符，需要进行转义：
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}"
    println!(" Hello \"{{World}}\" ");

    // 下面代码会报错，因为占位符{}只有一个右括号}，左括号被转义成字符串的内容
    // println!(" {{ Hello } ");
    // 也不可使用 '\' 来转义 "{}"
    // println!(" \{ Hello \} ")

    // 在格式化字符串时捕获环境中的值（Rust 1.58 新增）
    let person = get_person();
    println!("Hello, {person}!");
}

fn main() {
    // f1()
    // f2();
    // f3();
    // f4();
    f5();
}
