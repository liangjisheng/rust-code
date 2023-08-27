#![allow(unused)]
fn main() {
    let arr = [11, 22, 33, 44, 55];
    let s1 = &arr[0..3]; // [11,22,33]
    let s2 = &arr[1..=3]; // [22, 33, 44]
    let s3 = &arr[..]; // [11, 22, 33, 44, 55]

    println!("{:?}", arr);
    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);

    for i in 1..5 {
        println!("{}", i); // 1 2 3 4
    }

    // 下面两个表示范围的方式是等价的
    // let x = 0..5;
    let y = std::ops::Range { start: 0, end: 5 };
    println!("{:?}", y);

    assert_eq!((3..5), std::ops::Range { start: 3, end: 5 });
    assert_eq!(3 + 4 + 5, (3..6).sum());
}
