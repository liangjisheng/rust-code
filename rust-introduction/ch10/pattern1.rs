// Rust中为这两种匹配模式定义了专门的称呼：
// 不可反驳的模式(irrefutable)：一定会匹配成功，否则编译错误
// 可反驳的的模式(refutable)：可以匹配成功，也可以匹配失败，匹配失败的结果是不执行对应分支的代码

// let变量赋值、for迭代、函数传参这三种模式匹配只接受不可反驳模式。if let和while let只接受可反驳模式

// match则支持两种模式：
// 当明确给出分支的Pattern时，必须是可反驳模式，这些模式允许匹配失败
// 使用_作为最后一个分支时，是不可反驳模式，它一定会匹配成功
// 如果只有一个Pattern分支，则可以是不可反驳模式，也可以是可反驳模式

fn main() {
    // let变量赋值时使用可反驳的模式(允许匹配失败)，编译失败
    // let some_value = Some(5);
    // let Some(x) = some_value;

    // if let处使用了不可反驳模式，没有意义(一定会匹配成功)，给出警告
    if let x = 5 {
        // xxx
    }

    let value = Some(10);
    match value {
        Some(5) => (), // 允许匹配失败，是可反驳模式
        Some(50) => (),
        _ => (), // 一定会匹配成功，是不可反驳模式
    }

    match value {
        // 当只有一个Pattern分支时，可以是不可反驳模式
        x => println!("{}", x.unwrap()),
        _ => (),
    }

    // 模式部分可以是字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    // 模式带有变量名
    let x = (11, 22);
    let y = 10;
    match x {
        (22, y) => println!("Got: (22, {})", y),
        (11, y) => println!("y = {}", y), // 匹配成功，输出22
        _ => println!("Default case, x = {:?}", x),
    }
    println!("y = {}", y); // y = 10

    // 使用|可组合多个模式，表示逻辑或(or)的意思
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Rust支持数值和字符的范围, 但范围作为模式匹配的Pattern时，
    // 只允许使用全闭合的..=范围语法，其他类型的范围类型都会报错

    // 数值范围
    let x = 79;
    match x {
        0..=59 => println!("不及格"),
        60..=89 => println!("良好"),
        90..=100 => println!("优秀"),
        _ => println!("error"),
    }

    // 字符范围
    let y = 'c';
    match y {
        'a'..='j' => println!("a..j"),
        'k'..='z' => println!("k..z"),
        _ => (),
    }
}
