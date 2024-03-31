#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn c1() {
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 如果一个结构体中的所有字段都实现了 Copy 特征，那么这个结构体也会自动实现 Copy 特征
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    println!("p1 = ({}, {}), p2 = ({}, {})", p1.x, p1.y, p2.x, p2.y);

    // 数组也实现了 Copy 特征
    let a1 = [1, 2, 3];
    let a2 = a1;
    println!("a1 = {:?}, a2 = {:?}", a1, a2);

    // 如果一个元组中的所有元素都实现了 Copy 特征，那么这个元组也会自动实现 Copy 特征
    let t1 = (1, 2, 3);
    let t2 = t1;
    println!("t1 = {:?}, t2 = {:?}", t1, t2);

    // 字符串类型 String 并没有实现 Copy 特征，因为它是一个动态分配的类型。如果我们想要
    // 复制一个字符串，可以使用 clone 方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 枚举类型也可以实现 Copy 特征，但是需要注意枚举的所有成员都实现了 Copy 特征
    let c1 = Color::Red;
    let c2 = c1;
    println!("c1 = {:?}, c2 = {:?}", c1, c2);

    // 引用类型不实现 Copy 特征，因为它们是指向内存中的数据的指针。如果我们想要复制一个引用，
    // 需要使用 clone 方法。
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {:?}, v2 = {:?}", v1, v2);

    // 函数类型也不实现 Copy 特征，因为函数是代码的一部分，而不是数据。如果我们想要复制一个
    // 函数，可以使用指针或闭包。
    let f1 = add;
    let f2 = f1;
    println!("f1(2, 3) = {}, f2(2, 3) = {}", f1(2, 3), f2(2, 3));
}

#[derive(Copy, Clone)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn c2() {
    let r1 = Rectangle {
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point { x: 10, y: 10 },
    };
    let r2 = r1;
    println!(
        "r1 = ({}, {}), ({}, {}), r2 = ({}, {}), ({}, {})",
        r1.top_left.x,
        r1.top_left.y,
        r1.bottom_right.x,
        r1.bottom_right.y,
        r2.top_left.x,
        r2.top_left.y,
        r2.bottom_right.x,
        r2.bottom_right.y,
    );
}

fn main() {
    // c1();
    c2();
}
