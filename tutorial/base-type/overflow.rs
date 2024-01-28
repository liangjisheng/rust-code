// 假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生整型溢出。
// 关于这一行为 Rust 有一些有趣的规则：当在 debug 模式编译时，Rust 会检查整型溢出，若存在这些问题，
// 则使程序在编译时 panic(崩溃,Rust 使用这个术语来表明程序因错误而退出)。

// 在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。相反，当检测到整型溢出时，Rust
// 会按照补码循环溢出（two’s complement wrapping）的规则处理。

// 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如

fn main() {
    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b); // 19

    let b = a.checked_add(20);
    match b {
        Some(i) => println!("{}", i),
        None => println!("none"), // none
    }

    let (b, ok) = a.overflowing_add(20);
    if ok {
        println!("overflow b is {}", b);
    } else {
        println!("not overflow");
    }
}
