// Rust中必须得在条件判断处写返回值为true/false的表达式
// if x == 0 {
// ...
// }

// Rust的布尔值可以使用as操作符转换为各种数值类型，false 对应 0, true 对应 1
// 但数值类型不允许转换为bool值。再次提醒，as 操作符常用于原始数据类型之间的类型转换

fn main() {
    println!("{}", true as u32);
    println!("{}", false as u8);
    // println!("{}", 1_u8 as bool); // 编译错误
}
