// ..y 等价于 0..y
// x.. 等价于位置 x 到数据结束
// .. 等价于位置 0 到结束

// Rust 中有两种常用的字符串类型：str 和 String
// str 是 Rust 核心语言类型
// String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作
// String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性
// String 和 str 都支持切片，切片的结果是 &str 类型的数据

// 注意：切片结果必须是引用类型，但开发者必须自己明示这一点

fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    // s1 是一个 &str 类型的变量
    let s1 = "hello";
    println!("{}", s1);

    // 有一个快速的办法可以将 String 转换成 &str
    let s3 = &s[..];
    println!("{}", s3);

    // 除了字符串以外，其他一些线性数据结构也支持切片操作
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
