// Rust 中的 if 语句必须接收一个布尔值，不像 JavaScript 这门语言会自动转换，还可以省略括号

fn main() {
    let number = 1;
    if number < 2 {
        println!("true"); // true
    } else {
        println!("false");
    }

    // 如果预期不是一个布尔值，编译阶段就会报错
    // if number {
    //     println!("true");
    // }

    // 在 let 中使用 if 表达式，注意 if else 分支的数据类型要一致，因为 Rust 是静态类型，需要在编译期间确定所有的类型
    // 由于 if 是一个表达式，因此可以将其应用于语句中，如在 let 语句中使用 if 表达式来为其分配一个值
    // 由 {} 括起来的代码块返回的值或计算的结果取决于其中最后一个表达式，在此代码中 1 和 0 就是各自最后的表达式，
    // 因此两块代码块的结果正如表达式所述一样
    let condition = true;
    let num = if condition { 1 } else { 0 };
    println!("num: {}", num); // 1

    // Rust 会依次检检查每个 if表达式，并执行第一个条件成立的分支
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
