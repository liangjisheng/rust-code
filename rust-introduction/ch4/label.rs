// 可以为loop结构、while结构、for结构指定标签
// break和continue都可以指定标签来确定要跳出哪一个层次的循环结构

#![allow(unused)]
fn main() {
    // 'outer和'inner是标签名
    'outer: loop {
        'inner: while true {
            break 'outer; // 跳出外层循环
        }
    }

    // loop结构中的break可以同时指定标签和返回值，语法为break 'label RETURN_VALUE
    let x = 'outer: loop {
        'inner: while true {
            break 'outer 3;
        }
    };

    println!("{}", x); // 3
}
