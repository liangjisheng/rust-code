// char类型是Rust的一种基本数据类型，用于存放单个unicode字符，占用4字节空间(32bit)
// 在存储char类型数据时，会将其转换为UTF-8编码的数据(即Unicode代码点)进行存储。
// char字面量是单引号包围的任意单个字符，例如'a'、'我'。注意：char和单字符的字符串String是不同的类型

// 可使用as将char转为各种整数类型，目标类型小于4字节时，将从高位截断
// 可使用as将u8类型转char
// 可使用std::char::from_u32将u32整数类型转char，返回值Option<char>
// 可使用std::char::from_digit(INT, BASE)将十进制的INT转换为BASE进制的char

#![allow(unused)]
fn main() {
    // char -> Integer
    println!("{}", '我' as i32); // 25105
    println!("{}", '是' as u16); // 26159
    println!("{}", '是' as u8); // 47，被截断了

    // u8 -> char
    println!("{}", 97u8 as char); // a

    // std::char
    use std::char;

    println!("{}", char::from_u32(0x2764).unwrap()); // ❤
    assert_eq!(char::from_u32(0x110000), None); // true

    println!("{}", char::from_digit(4, 10).unwrap()); // '4'
    println!("{}", char::from_digit(11, 16).unwrap()); // 'b'
    assert_eq!(char::from_digit(11, 10), None); // true
}
