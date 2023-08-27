// Rust常用的字符串有两种:
// str：固定长度的字符串字面量
// String：可变长度的字符串对象

// str是Rust内置的字符串类型，它通常以引用的形式 &str 出现。字符串字面量 &str
// 是字符的集合，代表的是不可变的UTF-8编码的字符串的引用，创建后无法再为其追加内容或更改内容

// String是Rust标准库提供的、拥有所有权的UTF-8编码的字符串类型，创建后可以
// 为其追加内容或更改内容。String类型本质上是一个字段为Vec<u8>类型的结构体，
// 它把字符串内容存放在堆上，由指向堆上字节序列的指针(as_ptr方法)、记录堆上
// 字节序列的长度(len方法)和堆分配的容量(capacity方法)组成

fn reverse_words(str: &str) -> String {
    str.to_string()
        .split(" ")
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn f1() {
    let v = "alice bob";
    let rev = reverse_words(v);
    println!("{}", rev);
}

fn f2() {
    let mut s = String::from("Hello, ");
    s.push('R'); // 实际上是Vec<u8>的方法
    s.push_str("ust!");
    println!("{}", s);

    let mut s = String::from("Hello World!");
    s.insert(5, ',');
    s.insert_str(7, "Rust ");
    println!("{}", s); // Hello, Rust World!

    let s = format!("{}-{}", "Hello ", String::from("World"));
    println!("{}", s);

    let s = String::from("aaabbbbccaadd");
    let s1 = s.replace("aa", "77");
    let s2 = s.replacen("aa", "77", 1);
    println!("{}", s1);
    println!("{}", s2);

    // pop 删除并返回字符串的最后一个字符，返回值类型是Option<char>。如果字符串为空，则返回None。
    // remove 删除并返回字符串中指定位置的字符，其参数是该字符的起始索引位置。
    // remove方法是按字节处理字符串的，如果给定的索引位置不是合法的字符边界，将会导致程序错误。
    // truncate 删除字符串中从指定位置开始到结尾的全部字符，其参数是起始索引位置。
    // truncate方法也是按字节处理字符串的，如果给定的索引位置不是合法的字符边界，将会导致程序错误。
    // clear 等于与truncate方法的参数指定为0，删除字符串中的所有字符
    let mut s = String::from("hello rust world!");
    s.pop(); // Some('!')
    s.remove(9); // t
    s.truncate(9); // hello rus
    s.clear(); //

    let s = String::from("你好 世界");
    // 按字节迭代
    let bytes = s.bytes();
    for b in bytes {
        print!("{} | ", b);
    }
    println!();

    // 按字符迭代
    let chars = s.chars();
    for c in chars {
        print!("{} | ", c);
    }

    // 可以使用[]和一个范围(range)来创建String的切片
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 由于String是一个Vec<u8>，所以可以认为这个操作获取的是前4个字节
    println!("{}", s);

    // 当range的范围中有不完整的UTF8标量值，则会抛出panic
    let s = &hello[0..3];
    // println!("{}", s); // panic，因为第3个字节并不是一个UTF-8标量值的边界。
}

fn main() {
    // f1();
    f2();
}
