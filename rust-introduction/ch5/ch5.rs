// Rust中的引用是一种原始数据类型，它的位置认在栈中，保存的值是一种地址值，这个地址指向它所引用的目标
// 引用所指向的这个目标是 指向它所指向的那个变量(即指向位置)

#[derive(Debug)]
struct User {
    vip: VIP,
}

#[derive(Debug)]
enum VIP {
    VIP0,
    VIP1,
    VIP2,
    VIP3,
}

fn main() {
    let user = User { vip: VIP::VIP0 };
    match user.vip {
        VIP::VIP0 => println!("not a vip"),
        // "声明"了变量a，发生 move
        // a => println!("vip{:?}", a),
        ref a => println!(), // 借用而不 move
    }
    println!("{:?}", user); // 不使用 ref 会报错

    // 在上面的match匹配代码中，第二个分支使用了变量a，尽管匹配时会匹配第一个分支，
    // 但Rust编译器并不知道匹配的结果如何，因此编译器会直接move整个user到这个分支

    // 位置一旦初始化赋值，就会有一个永远不变的地址，直到销毁。换句话说，变量一旦初始化，
    // 无论它之后保存的数据发生了什么变化，它的地址都是固定不变的。也说明了，
    // 编译器在编译期间就已经安排好了所有位置的分配
    let mut n = "hello".to_string(); // n是一个栈中的位置，保存了一个胖指针指向堆中数据
    println!("n: {:p}", &n); // &n产生一个位置，该位置中保存指向位置n的地址值

    let m = n; // 将n中的胖指针移给了m，m保存胖指针指向堆中数据，n变回未初始化状态
    println!("m: {:p}", &m); // &m产生一个位置，该位置中保存指向位置m的地址值

    n = "world".to_string(); // 重新为n赋值，位置n保存另一个胖指针，但位置n还是那个位置
    println!("n: {:p}", &n); // &n产生一个位置，该位置中保存指向位置n的地址值
}

// 在Rust中，赋值操作，实际上是一种值的移动：将值从原来的位置移入到目标位置
// 如果类型实现了Copy trait，则Copy而非Move
