// 引用与租借, 如果你熟悉指针的概念，你可以把它看作一种指针 实质上"引用"是变量的间接访问方式
// 当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
// 引用不会获得值的所有权,引用只能租借（Borrow）值的所有权
// 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权

fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 因为 s2 租借的 s1 已经将所有权移动到 s3，所以 s2 将无法继续租借使用 s1 的所有权
    // 如果需要使用 s2 使用该值，必须重新租借
    // let s1 = String::from("hello");
    // let s2 = &s1;
    // let s3 = s1;
    // println!("{}", s2);

    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    // 如果尝试利用租借来的权利来修改数据会被阻止
    // let s4 = String::from("run");
    // let s5 = &s4;
    // println!("{}", s5);
    // s5.push_str("oob"); // 错误，禁止修改租借的值
    // println!("{}", s5);

    let mut s4 = String::from("run"); // s4 是可变的
    let s5 = &mut s4; // s5 是可变的引用
    s5.push_str("oob");
    println!("{}", s5);

    // 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以
    // let r1 = &mut s;
    // let r2 = &mut s;

    // 垂悬引用, 这是一个换了个名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有
    // 实际指向一个真正能访问的数据的指针（注意，不一定是空指针，还有可能是已经释放的资源）
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 很显然，伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了
// 但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// Rust 对栈内存和堆内存一视同仁，超出作用域一律自动释放
// Rust 的这个特点在兼顾性能的情况下、有效的减少了代码量和内存泄漏隐患
// 这部分对于所有权的设定，看起来很奇怪，其实本质上就是在语言层面禁止了同一个可变数据会
// 有多个变量引用的情况，一旦作为参数传递了，就会发生所有权的移动（Move）或借用（Borrow）
// 赋值给另一个变更也就自动放弃了所有权。从根本上杜绝了并发情景下的数据共享冲突
