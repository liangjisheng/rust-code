#![allow(unused)]
fn main() {
    let mut x = 0;
    while x < 5 {
        x += 1;
        println!("{}", x);
        if x % 2 == 0 {
            continue;
        }
    }

    // COND部分允许加入其他语句，只要COND部分最后返回bool类型即可
    let mut x = 0;
    // 相当于do..while
    while {
        println!("{}", x);
        x < 5
    } {
        x += 1;
        if x % 2 == 0 {
            continue;
        }
    }

    // while虽然有默认返回值()，但()作为返回值是没有意义的。因此不考虑 while 的返回值问题
}
