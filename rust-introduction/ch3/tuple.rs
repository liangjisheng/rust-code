// Rust的tuple类型可以存放0个、1个或多个任意数据类型的数据。使用tup.N的方式可以访问索引为N的元素
// 访问tuple元素的索引必须是编译期间就能确定的数值，而不能是变量

#![allow(unused)]
fn main() {
    let n = (11, 22, 33);
    println!("{}", n.0); // 11
    println!("{}", n.1); // 22
    println!("{}", n.2); // 33

    let p = ("alice", 23); // 同时存放&str和i32类型的数据
    println!("{}, {}", p.0, p.1);

    // 也可以类型推导：let (name,age) = p;
    let (name, age): (&str, i32) = p;
    // 比 let name = p.0; let age = p.1; 更简洁
    println!("{}, {}", name, age);

    // 有时候tuple里只会保存一个元素，此时必须不能省略最后的逗号
    let p = ("alice",);

    // 不保存任何数据的tuple表示为()。在Rust中，它是特殊的，它有自己的类型：unit
    // unit类型的写法为()，该类型也只有一个值，写法仍然是()
    // 将值()保存到类型为()的变量x中
    //    类型    值
    let x: () = ();
    // unit 类型通常用在那些不关心返回值的函数中
}
