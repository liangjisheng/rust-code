// 关于函数的几个特点总结如下
// 使用 fn 关键字声明
// 函数参数必须定义类型
// 箭头 -> 后声明返回类型，默认情况下返回最后一个表达式，注意不要有分号 ;
// 也可使用 return 返回，这里要加分号 ;

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// 在函数定义中要求类型描述意味着编译器几乎不需要在代码的其它地方来确定开发者的意图。
// 如果编译器知道函数期望的类型，它还能够提供更多有用的错误信息
fn another_function(x: i32, s: &str) {
    println!("The value of x is: {x}, str is {s}");
}

// 语句： 用来执行某些操作且不返回值的操作指令
// 表达式： 用来计算并以计算的结果为返回值的操作
// main 函数的声明也是语句，即 fn main() {...}整体可视为一条语句
// 表达式可以是作为语句的一部分
// 调用函数是一个表达式、调用宏也是一个表达式、使用大括号创建的一个新范围块也是一个表达式

// 在 Rust 中函数的返回值与函数体块中的最终表达式的值同义
fn five() -> i32 {
    // 表达式 5
    5
}

fn f1() {
    let res1 = multiply(2, 3);
    let res2 = add(2, 3);
    print!("multiply {}, add {} \n", res1, res2);
    another_function(5, "hello");

    let y = {
        let x = 3;
        x + 1
    };
    // 这是由大括号括起来的一个范围块，该条语句的计算结果为 4，该语句计算的结果将会绑定到 y 上，
    // 成为整条变量定义语句的一部分。仔细看 x + 1 这一行的末尾并没有分号，与其他大多数代码行不同。
    // 表达式不包括结束分号，如果在表达式的末尾添加分号，则会将其转换为语句，此时将不会返回值
    println!("The value of y is: {y}");

    println!("five: {}", five());
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn f2() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn main() {
    // f1();
    f2();
}
