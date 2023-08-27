// 切片(Slice)是Rust的一种不持有所有权的数据类型。通常切片拥有一个指针指向一个容器，
// 同时拥有一个范围用于规定通过切片可访问到的容器中元素的范围。切片是一种胖指针。

// 字符串s拥有三个元素：指向heap数据的指针、字符串长度、字符串容量
// 字符串切片world拥有两个元素：指向heap数据起始位置的指针、切片长度
fn s1() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    // 字符串字面值是切片
    let s1 = "hello";
}

// 获取字符串中第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn s2() {
    let s = String::from("hello world");
    let world = first_word(&s);
    println!("{}", world);
}

fn main() {
    // s1();
    s2();
}

// 建议传给函数的字符串引用参数以切片的形式给出，即&str，这样可以保证函数
// 既可以接收到&String类型的参数，也可以接收&str类型的参数。
// 注意：&String和&str是两个不同的类型：
// &String 是字符串值的引用
// &str 是字符串值的切片

// 对于一个以 &str 为入参的函数：fn first_word(s: &str) -> &str { /*...*/ }
// 当入参是一个 &String 类型时，会创建一个完整的切片来调用该函数，即 &str_name[..]
// 当入参是一个字符串切片(&str)时，会直接调用该函数。
// 定义函数时使用字符串切片来代替字符串引用，可以是API更加通用，且不会损失任何功能
