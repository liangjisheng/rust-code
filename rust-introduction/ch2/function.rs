// 函数也可以直接定义在函数内部。例如在函数a中定义函数b，这样函数b就只能在函数a中访问或调用

fn f0() {
    println!("first function_0");
    println!("first function_1");

    fn f1(a: i32, b: i32) {
        println!("a: {}, b: {}", a, b);
    }

    f1(2, 3);
}

// Rust 有两种方式指定函数返回值
// 使用return来指定返回值，此时return后要加上分号结尾，使得return成为一个语句
// return关键字不指定返回值时，默认返回()
// 不使用return，将返回最后一条执行的表达式计算结果，该表达式尾部不能带分号
// 不使用return，但如果最后一条执行的是一个分号结尾的语句，则返回()

fn f2() -> i32 {
    // 分号不能少
    return 1;
}

fn f3() {
    // return ()
    return;
}

fn f4() -> i32 {
    //如果想返回 1, 则结尾不能有分号
    1
}

fn f5() {
    // return ()
    2;
}

fn main() {
    // f0();

    println!("{:?}", f2());
    println!("{:?}", f3());
    println!("{:?}", f4());
    println!("{:?}", f5());
}
