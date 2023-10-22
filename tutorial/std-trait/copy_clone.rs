// trait Copy:Clone{}
// Copy 是 Clone 的 subtrait，所以一个类型如果要实现 Copy 的话，必须实现 Clone
// 所以下面的代码报错, 不能单独继承 Copy

// #[derive(Copy, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// Copy承诺拷贝操作是简单的按位拷贝，所以它是快速高效的。我们不能自己实现Copy，
// 只有编译器可以提供实现，但是我们可以通过使用Copy派生宏让编译器这么做，就像
// 使用Clone派生宏一样，因为Copy是Clone的一个 subtrait

#[derive(Clone, Debug)]
struct Point1 {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

fn f1() {
    // 没有实现 Copy, p 赋值后会失效
    let p = Point1 { x: 0, y: 1 };
    let pp = p;
    // println!("p {:?}", p); // error 使用移动过的值
}

fn f2() {
    // 实现了 Copy, p 赋值后不会失效
    let p = Point2 { x: 0, y: 1 };
    let pp = p;
    println!("p {:?}", p);
    println!("pp {:?}", pp);
}

fn main() {
    // f1();
    f2();
}
