// Rust 语言为了保证安全性，尽可能抛弃指针，也就是默认会把所有的数据都存储在 栈 上
// 如果要把数据存储在 堆 上，就要在 堆 上开辟内存，这时候就要使用到 指针。
// 作为系统级的语言， Rust 提供了在 堆 上存储数据的能力。只不过它把这些能力弱化并封装到了 Box 中。
// 这种把 栈 上数据搬到 堆 上的能力，我们称之为 装箱。
// Rust 语言中的某些类型，如 向量 Vector 和 字符串对象 String 默认就是把数据存储在 堆 上的

// Box 指针也称之 装箱 (box), 允许我们将数组存储在 堆 (heap) 上而不是 栈 (stack) 上
// 但即使把数据存储在 堆 (heap) 上，栈 (stack) 仍然包含了指向 堆数据的指针
// Box 指针没有任何额外的其它开销，因为它仅仅只是把数据存储在 堆 (heap) 而已

// 所以大多数时候我们会基于以下几个条件来判断是否需要用到 Box 类型来保存数据
// (1) 占用大空间的数据对象：避免方法栈替换时的数据传输开销
// (2) 递归类型定义：编译时无法确定数据大小，则以固定大小的栈指针代替

fn b1() {
    let x = 5; // 默认保存在 栈 上
    let y = Box::new(x); // 使用 Box 后数据会存储在堆上
    println!("y = {}", y);

    // 当我们使用 Box::new() 把一个数据存储在堆上之后，为了访问存储的具体数据，我们必须 解引用。
    // 解引用 需要使用操作符 星号 *，因此 星号 * 也称之为 解引用操作符
    println!("{}", 5 == x);
    println!("{}", 5 == *y); // 为了访问 y 存储的具体数据，需要解引用
}

// Deref 是由 Rust 标准库提供的一个 特质 ( trait )。
// 实现 Deref 特质需要我们实现 deref() 方法。
// deref() 方法从某些方面说用于借用 self 对象并返回一个指向内部数据的指针。
// 也就是说 deref() 方法返回一个指向结构体内部数据的指针

use std::ops::Deref;
use std::ops::Drop;

// 元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 范型方法
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0 // 返回数据
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping MyBox object from memory ");
    }
}

fn b2() {
    let x = 5;
    let y = MyBox::new(x); // 调用静态方法 new() 返回创建一个结构体实例

    println!("5==x is {}", 5 == x);
    println!("5==*y is {}", 5 == *y); // 解引用 y
    println!("x==*y is {}", x == *y); // 解引用 y
}

// Drop Trait 只有一个方法 drop()
// 当实现了 Drop Trait 的结构体在离开了它的作用域范围时会触发调用 drop() 方法
// 一些其它语言中，比如 C++，智能指针每次使用完了之后都必须手动释放相关内存或资源。
// 而在 Rust 语言中，我们可以把释放内存和资源的操作交给 Drop trait
fn b3() {
    let x = 50;
    MyBox::new(x);
    MyBox::new("Hello");
}

// 避免栈上数据的拷贝
fn b4() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

// Box 中还提供了一个非常有用的关联函数：Box::leak，它可以消费掉 Box 并且强制目标值从内存中泄漏
// 而通过 Box::leak 我们不仅返回了一个 &str 字符串切片，它还是 'static 生命周期的！
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    let r = s.into_boxed_str();
    Box::leak(r)
}

fn b5() {
    let s = gen_static_str();
    println!("{}", s);

    let x = Box::new(41);
    let static_ref: &'static mut usize = Box::leak(x);
    *static_ref += 1;
    assert_eq!(*static_ref, 42);

    let x = vec![1, 2, 3].into_boxed_slice();
    let static_ref = Box::leak(x);
    static_ref[0] = 4;
    assert_eq!(*static_ref, [4, 2, 3]);
}

fn main() {
    // b1();
    // b2();
    // b3();
    // b4();
    b5();
}
