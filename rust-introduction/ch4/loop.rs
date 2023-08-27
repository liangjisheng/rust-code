// loop表达式是一个无限循环结构。只有在loop循环体内部使用break才能终止循环。
// 另外，也使用continue可以直接跳入下一轮循环

#![allow(unused)]
fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    }

    // loop也有默认返回值()，可以将其赋值给变量
    // let a = loop {
    //     ...
    // };
    // println!("{:?}", a);   // ()

    // 作为一种特殊情况，当在loop中使用break时，break可以指定一个loop的返回值
    // ----------------------------------------------------------------
    let mut x = 0;
    let a = loop {
        x += 1;
        if x == 5 {
            break x; // 返回跳出循环时的x，并赋值给变量 a
        }
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    };
    println!("var a: {:?}", a); // 输出 var a: 5
}
