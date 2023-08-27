// 向量是线性表，在 Rust 中的表示是 Vec<T>
// let vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的空向量
// let vector = vec![1, 2, 4, 8];     // 通过数组创建向量

fn main() {
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    // append 方法用于将一个向量拼接到另一个向量的尾部
    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

    let mut v = vec![1, 2, 4, 8];
    println!(
        "{}",
        match v.get(0) {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        }
    );
    println!("{}", v[1]);

    for i in &v {
        println!("{}", i);
    }

    // 遍历过程中需要更改变量的值
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}
