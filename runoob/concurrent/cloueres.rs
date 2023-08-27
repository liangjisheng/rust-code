// 闭包
// |参数1, 参数2, ...| -> 返回值类型 {
//     函数体
// }

fn main() {
    let inc = |num: i32| -> i32 { num + 1 };
    println!("inc(5) = {}", inc(5));

    // 闭包可以省略类型声明使用 Rust 自动类型判断机制
    let inc = |num| num + 1;
    println!("inc(5) = {}", inc(5));
}
