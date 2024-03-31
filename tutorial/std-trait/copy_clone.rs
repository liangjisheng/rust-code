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

// 并不是所有类型都可以实现 Copy trait。只有满足以下条件的类型才能实现 Copy：
// 类型本身是 POD（Plain Old Data）类型，即不包含任何指针或引用。
// 类型的所有字段都实现了 Copy

// 为什么需要 Copy trait
// Copy trait 允许你控制类型的复制行为。当一个类型实现了 Copy trait 时，它的值
// 可以在赋值、传参和返回值时自动复制。这样，你就可以避免显式调用 clone 方法来复制值。
// 此外，由于 Copy 类型的值总是按位复制，所以它们的复制开销很小。这对于提高程序性能非常有帮助

// 什么类型可以实现 Clone trait
// 几乎所有类型都可以实现 Clone trait。只要你能够定义如何创建一个新的副本，你就可以
// 实现 Clone trait
// 为什么需要 Clone trait
// Clone trait 允许你显式地复制类型的值。这对于那些不能按位复制的类型非常有用，例如
// 包含指针或引用的类型。
// 此外，Clone trait 还允许你自定义复制行为。你可以在 clone 方法中添加任何逻辑，以便
// 在复制时执行特定的操作

// 此外，所有实现了 Copy 的类型都必须实现 Clone。这是因为当你显式地调用 clone 方法时，
// Rust 会假定你知道自己在做什么，并且希望按位复制该值

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
