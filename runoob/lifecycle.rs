fn main() {
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
    }
    println!("{} is longer", r);
}

// r 被使用的时候源值 s1 和 s2 都已经失效了, 编译失败
// fn longer(s1: &str, s2: &str) -> &str {

// &i32        // 常规引用
// &'a i32     // 含有生命周期注释的引用
// &'a mut i32 // 可变型含有生命周期注释的引用

// 需要用泛型声明来规范生命周期的名称，随后函数返回值的生命周期将与两个参数的生命周期一致
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}
