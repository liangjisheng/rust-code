fn main() {
    // 不可变变量
    let a = 12;
    // a = "abc";
    // a = 4.56;
    // a = 456;
    println!("a is {}", a);

    // 可变变量 b
    let mut b = 123;
    b = 456;
    println!("b is {}", b);

    println!("a is {}, a again is {}", a, a);
    //在 {} 之间可以放一个数字，它将把之后的可变参数当作一个数组来访问，下标从 0 开始
    println!("a is {0}, a again is {0}", a);
    // 如果要输出 { 或 }
    println!("{{}}");

    // 重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。但可变变量赋值仅能发生值的变化
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // let mut s = "123";
    // s = s.len();

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余

    // 元组是用一对 ( ) 包括的一组数据，可以包含不同种类的数据
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
}
