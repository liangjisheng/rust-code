// std::ops::Range
// std::ops::RangeFrom
// std::ops::RangeTo
// std::ops::RangeFull
// std::ops::RangeInclusive
// std::ops::RangeToInclusive

#![allow(unused)]
fn main() {
    let arr = [11, 22, 33, 44, 55];
    let s1 = &arr[0..3]; // [11,22,33]
    let s2 = &arr[1..=3]; // [22, 33, 44]
    let s3 = &arr[..]; // [11, 22, 33, 44, 55]

    // 范围表达式也常被用于迭代操作
    for i in 1..5 {
        println!("{}", i); // 1 2 3 4
    }

    // 范围表达式和对应类型的实例是等价的。例如，下面两个表示范围的方式是等价的
    let x = 0..5;
    let y = std::ops::Range { start: 0, end: 5 };
    assert_eq!(x, y);
}
