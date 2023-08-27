// 可以通过引用或多次引用，使用或改变原始变量的值（需要手动多次解引用）
fn b1() {
    let mut a = Box::new(5);
    let mut b = &mut a;
    *b = Box::new(12);
    println!("b = {:?}", b);
    // cannot borrow `a` as immutable because it is also borrowed as mutable
    // 当不可变引用使用后, 不可变引用,可变引用,值 同时只能存在一个
    // println!("a = {:?}", a);

    let c = &mut b;
    **c = Box::new(10);
    println!("c = {:?}", c);
}

// 只改变Box里面的值, box 可以视为是一个指向堆数据的指针
fn b2() {
    let mut a = Box::new(5);
    *a += 12;
    println!("a = {:?}", a); // 17

    // 支持多重连续引用
    let mut b = &mut a;
    **b += 10;
    println!("b = {:?}", b); // 27
    println!("a = {:?}", a); // 17
}

#[derive(Debug)]
struct Point {
    x: Box<i32>,
    y: Box<i32>,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point {
            x: Box::new(x),
            y: Box::new(y),
            z,
        }
    }
}

// 如果 box 其中包含的是复合类型，也可以通过解引用改变其中的值，该操作支持多重引用
fn b3() {
    let mut a = Box::new(Point::new(5, 5, 9));
    *a.x += 10; // because a.x's type is Box<i32>, it needs to deref.
    a.z += 10; // a.z's type is i32, it doesn't need to deref.
    println!("a = {:?}", a); // { x: 15, y: 5, z: 19 }

    let b = &mut a;
    *b.x += 10;

    println!("b = {:?}", b); // { x: 25, y: 5, z: 19 }
    println!("a = {:?}", a); // { x: 25, y: 5, z: 19 }
}

// Box 指针的解引用
fn b4() {
    let mut a = Box::new(5);
    // 对于简单类型来说，其值是copy的
    let b = *a; // copy '*a' to b.

    *a += 12;
    println!("b = {:?}", b); // 5
    println!("a = {:?}", a); // 17

    // 对于简单类型来说，引用情况下，允许对引用进行解引用吗？答案是不允许的
    let a1 = Box::new(5);
    let b1 = &a1;
    // cannot move out of `*b1` which is behind a shared reference
    // 这里注意，b为a的引用，其类型为&Box<i32>，对b进行解引用的话，*b的类型才为 Box<i32>，
    // 如上错误提示，*b 未实现Copy trait, 并且*b也不拥有其指向变量a的所有权，
    // 所以无法移动到变量c中，违反所有权规则，报错。可变引用情况下也是如此
    // let c1 = *b1;
}

// 在这种情况下，可以一直解引用到其包含的值，并进行操作，但此时的值是copy的
// 可以看出，对c的操作不改变a和b，是可以的，如下代码
fn b5() {
    let mut a = Box::new(5);
    let b = &mut a;
    let mut c = **b;
    c += 20;
    **b += 10;

    println!("c = {:?}", c); // 25
    println!("b = {:?}", b); // 15
    println!("a = {:?}", a); // 15
}

// 复合类型解引用
fn b6() {
    let a = Box::new(Point::new(1, 2, 3));
    // 对a进行解引用后，将a包含的值的所有权转移到了变量b中（因为Point未实现Copy trait）
    // 同时a变得无效，不可再次使用。此时，可以对b进行各种操作
    let mut b = *a;

    println!("b = {:?}", b);
    // println!("a = {:?}", a);

    *b.x += 10;
    b.z += 10;
    println!("b = {:?}", b);
}

fn b7() {
    let a = Box::new(Point::new(5, 5, 5));
    let b = &a;
    // 引用情况下，允许对引用进行解引用吗？答案也是不允许的
    // move occurs because `*b` has type `Box<Point>`, which does not implement the `Copy` trait
    // let c = *b;

    // 一直解引用到其包含的值，并进行move操作吗？经过实验，是不行的
    // move occurs because `*b` has type `Box<Point>`, which does not implement the `Copy` trait
    // let c = **b;
}

// Box包含struct成员的所有权的转移
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>, // 'next' has the ownership of the next node.
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, next: None })
    }

    fn link(&mut self, node: Box<Node>) {
        self.next = Some(node);
    }
}

fn b8() {
    let mut a = Node::new(1);
    let b = Node::new(2);
    a.link(b);

    // a的类型为 Box<Node>，但代码仍然可以将a.next 的所有权 move 到 变量 c 中
    // 等价于 let c = (*a).next;
    let c = a.next;
    println!("{:?}", c);
    // value borrowed here after partial move
    // println!("{:?}", a);

    let d = Node::new(1);
    let e = &d;
    // e是Node的引用，无权转移其成员的所有权
    // move occurs because `e.next` has type `Option<Box<Node>>`, which does not implement the `Copy` trait
    // let f = e.next;
}

fn main() {
    // b1();
    // b2();
    // b3();
    // b4();
    // b5();
    // b6();
    // b7();
    b8();
}
