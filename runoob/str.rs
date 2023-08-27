fn main() {
    let one = 1.to_string(); // 整数到字符串
    let float = 1.3.to_string(); // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串

    println!("{}", one);
    println!("{}", float);
    println!("{}", slice);

    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!'); // 追加字符
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 使用 format! 宏
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // 字节长度
    println!("{}", s.len());
    // 先取字符串为字符集合，再计算字符长度
    let c_len = s.chars().count();
    println!("{}", c_len);

    for c in s.chars() {
        println!("{}", c);
    }
}
