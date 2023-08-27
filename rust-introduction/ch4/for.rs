// Rust数组不支持迭代，要迭代数组各元素，需将数组转换为Slice再进行迭代

#![allow(unused)]
fn main() {
    // 迭代Range类型：1..5
    for i in 1..5 {
        println!("{}", i);
    }

    let arr = [11, 22, 33, 44];
    // arr是数组，&arr转换为Slice，Slice可迭代
    for i in &arr {
        println!("{}", i);
    }
}
