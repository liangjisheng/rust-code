// Box 是 Rust 使用堆的一种方式，而 C++ 是 new，C 是 malloc。使用它们的原因基本是一致的
// 栈的尺寸相对小，如果数据尺寸太大，栈可能会爆掉
// 大尺寸数据在栈上传递拷贝可能会影响程序性能
// 使用 dyn 多态时，必须使用引用或智能指针，而普通引用容易遇到生命周期的问题，所以有时必须使用智能指针

// 最简单直接的智能指针是box，其类型是Box<T>。box允许你将一个值放在堆上而不是栈上。
// 留在栈上的则是指向堆数据的指针
// Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待
// 当Box<T>值离开作用域时，由于Box<T>类型 Drop trait的实现，box所指向的堆数据也会被清除

// 使用Box指针的递归类型
// 在任何编程语言中，编译器都需要知道任何数据类型占用的空间。这对于 Rust Compiler 来说是一样的
// 但是在递归 类型的情况下，这是相当困难的，递归类型其中值可以具有作为自身的一部分的另一值相同类型
// 编译器不知道在编译时为这种数据类型分配多少内存
// 所有不能在编译时确定空间大小的数据，全部都要放置到堆区

// 报错: recursive type `RecType` has infinite size
// 编译器不知道要为RecType分配多少空间
// enum RecType {
//     Cons(i32, RecType),
//     Nil,
// }

// 可以使用Box 指针。由于我们可以知道Box 指针在编译时占用的空间
// 因此我们可以在元组中使用Box 指针来指向RecType类型的值
#[derive(Debug)]
enum RecType {
    Cons(i32, Box<RecType>),
    Nil,
}

fn b1() {
    // 这里定义了变量 b, 其值是一个指向被分配在堆上的值5的 Box
    // 当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放
    // 这个释放过程作用于 box 本身(位于栈上)和它所指向的数据（位于堆上）
    let b = Box::new(5);
    println!("b is {}", b);

    let ptr = Box::new("hello");
    println!("The pointer points to data : {}", ptr);

    let data = RecType::Cons(
        1,
        Box::new(RecType::Cons(
            2,
            Box::new(RecType::Cons(3, Box::new(RecType::Nil))),
        )),
    );
    println!("{:?}", data);
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Box<Option<Node>>,
}

fn b2() {
    let mut node = Node {
        val: 1,
        next: Box::new(Some(Node {
            val: 2,
            next: Box::new(None),
        })),
    };
    println!("node = {:?}", node);
    *node.next = None;
    println!("node = {:?}", node);
}

fn main() {
    // b1();
    b2();
}
