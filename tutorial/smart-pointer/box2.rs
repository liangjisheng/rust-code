use std::mem::size_of_val;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// 使用 std::mem::size_of_val 来区别 Box 指针与一般栈上变量的占用空间区别

fn b1() {
    // Point on stack
    let point = Point { x: 0, y: 0 };
    println!("point = {:?}", point);
    println!("sizeof(point) = {}", size_of_val(&point)); // 8

    // Rectangle on stack
    let point_b = Point { x: 10, y: 10 };
    let rect = Rectangle {
        top_left: point,
        bottom_right: point_b,
    };
    println!("rect = {:?}", rect);
    println!("sizeof(rect) = {}", size_of_val(&rect)); // 16
    println!();

    // Point on heap
    let box_point = Box::from(point);
    println!("box_point = {:?}", box_point);
    println!("sizeof(box_point) = {}", size_of_val(&box_point)); // 8

    // Rectangle on heap
    let box_rect = Box::from(rect);
    println!("box_rect = {:?}", box_rect);
    println!("sizeof(box_rect) = {}", size_of_val(&box_rect)); // 8

    // Rectangle 没有 clone trait, 所以 Box::from() 会取得 rect 的所有权
    println!("rect = {:?}", rect);
    println!();
}

fn b2() {
    let mut point = Point { x: 0, y: 0 };
    // 可以直接对 Box 的指向的变量重新赋值（注意这里由于 point 实现了 Clone 特性，
    // 所以实际上 box_point 指向了另一个 Point 对象，这点可以从 Box<Point> 类型看出
    let mut box_point = Box::from(point);
    println!("point = {:?}", point);
    println!("box_point = {:?}", box_point);

    // assign 'mut Box<Point>'
    box_point.x = 1;
    box_point.y = 1;
    println!("point = {:?}", point);
    println!("box_point = {:?}", box_point);

    // 还可以修改 Box 的指针，让他指向一个新的 Point 对象
    *box_point = Point { x: 2, y: 2 };
    println!("point = {:?}", point);
    println!("box_point = {:?}", box_point);

    let box_point_ref = Box::from(&mut point);
    box_point_ref.x = 3;
    box_point_ref.y = 3;
    println!("box_point_ref = {:?}", box_point_ref);
    println!("point = {:?}", point);

    // 这里就要注意所谓的引用所有权问题，Box 变量持有了原 point 变量的可变引用(&mut)
    // 因此在这个 box_point_ref 使用完毕之前都不能再使用任何其他引用(&point)
    // 这也是为什么需要把 print 的顺序反过来
}

fn main() {
    b1();
    // b2();
}
