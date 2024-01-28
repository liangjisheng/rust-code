// https://juejin.cn/post/7208092574162288698

// impl Trait 的返回值类型并不支持多种不同的类型返回，那如果我们想返回多种类型
// 该怎么办？这时候就需要用到 trait 对象

// trait 是一种约束，而不是具体类型，它属于DST（类似于 str），其 size 无法在编译阶段确定
// 只能通过指针来间接访问，可以通过 &Trait 引用或者 Box<Trait> 智能指针的方式来使用。

// Rust 中用一个胖指针（除了一个指针外，还包含另外一些信息），来表示 一个指向 trait 的引用
// (类似于str和&str)，即 trait object，分别指向 data 与 vtable。简单讲，trait object
// 包含 data 指针和 vtable 指针

// 什么是 Trait Object 呢？指向 trait 的指针就是Trait Object。假如 Summary 是一个
// trait 的名称，那么 &Summary 和 Box<Summary> 都是一种 Trait Object，是“胖指针“

// vtable是virtual method table 的缩写。 本质上是结构体指针的结构，指向具体实现中每种
// 方法的一段机器代码。在 Rust 中，当使用多态性时，编译器会自动为每个带有虚函数的类型创建
// 一个虚表。在运行时，这个虚表会被动态分配内存，并用于存储虚函数的地址。虚拟表只在编译时
// 生成一次，由同类型的所有对象共享

// dyn 关键字只用在 trait 对象的类型声明上，常见形式有 Box<dyn trait> &dyn trait等

use std::f64::consts::PI;

// 圆形
struct Circle {
    radius: f64,
}

// 正方形
struct Square {
    side: f64,
}

/// 声明一个图形 shape trait
trait Shape {
    fn area(&self) -> f64;
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

/// 为 Circle 实现 Shape
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

/// 为 Square 实现 Shape
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// 计算 trait 对象列表面积之和
fn get_total_area(shapes: Vec<Box<dyn Shape>>) -> f64 {
    shapes.into_iter().map(|s| s.area()).sum()
}

fn t1() {
    let circle = Circle::new(5.0); // 实现了Shape trait 的结构体
    let shape: &dyn Shape = &circle; // &circle 是 trait 对象，用&dyn Shape申明
    println!("shape => {}", shape.area());

    // 特性对象也允许我们在集合中存储不同类型的值：
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }), // Box<Circle> cast to Box<dyn Shape>
        Box::new(Square { side: 1.0 }),   // Box<Square> cast to Box<dyn Shape>
    ];
    assert_eq!(PI + 1.0, get_total_area(shapes)); // ✅
}

// trait 对象执行动态分发
// Rust可以同时支持“静态分派(static dispatch)”和“动态分派(dynamic dispatch)”

// 静态分发：指的是具体调用哪个函数，在编译阶段就确定下来了。Rust中的“静态分派”靠泛型
// (impl Trait是泛型的语法糖)来完成。对于不同的泛型类型参数，编译器会执行的单态化处理，
// 为每一个被泛型类型参数代替的具体类型生成不同版本的函数和方法，在编译阶段就确定好了应
// 该调用哪个函数。

// 动态分派：指的是具体调用哪个函数，在执行阶段才能确定。Rust中的“动态分派”靠
// Trait Object 来完成。Trait Object 本质上是指针，它可以指向不同的类型，
// 指向的具体类型不同，调用的方法也就不同

// 只有对象安全（object-safe）的 trait 可以实现为特征对象。这里有一些复杂的规则来实现
// trait 的对象安全，但在实践中，如果一个 trait 中定义的都符合以下规则，则该 trait 是对象安全的：
// 方法的返回类型不能是Self
// 方法没有任何泛型参数
// Trait 不能Sized约束

// 这主要因为把一个对象转为 trait object 后，原始类型信息就丢失了，所以这里的 Self
// 也就无法确定了，那么该方法将不能使用原本的类型。当 trait 使用具体类型填充的泛型
// 类型时也一样：具体类型成为实现 trait 的对象的一部分，当使用 trait 对象时其具体类型
// 被抹去了，无法知道应该用什么类型来填充泛型类型

// 如果说泛型给了我们编译时的多态性，那么 trait 对象就给了我们运行时的多态性。
// 通过 trait 对象，我们可以允许函数在运行时动态地返回不同的类型。trait 对象的结构体
// 大小是未知的，所以必须要通过指针来引用它们。具体类型与 trait 对象在字面上的区别在于，
// trait 对象必须要用 dyn 关键字来修饰前缀

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    let r1 = x.draw();
    println!("{}", r1);
}

fn draw2(x: &dyn Draw) {
    let r1 = x.draw();
    println!("{}", r1);
}

fn t2() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}

// 注意 dyn 不能单独作为特征对象的定义，例如下面的代码编译器会报错，原因是特征对象可以是任
// 意实现了某个特征的类型，编译器在编译期不知道该类型的大小，不同的类型大小是不同的。
// 而 &dyn 和 Box<dyn> 在编译期都是已知大小，所以可以用作特征对象的定义。
// 编译报错
// fn draw3(x: dyn Draw) {
//     x.draw();
// }

fn main() {
    // t1();
    t2();
}
