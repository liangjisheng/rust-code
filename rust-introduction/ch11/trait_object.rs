// https://rust-book.junmajinlong.com/ch11/04_trait_object.html

// 当B、C、D类型实现了Trait A后，就可以将类型B、C、D当作Trait A来使用。
// 这在概念上来说似乎是正确的，但根据Rust语言的特性，Rust没有直接实现这样的用法。
// 原因之一是，Rust中不能直接将Trait当作数据类型来使用

// Audio类型实现了Trait Playable，在创建Audio实例对象时不能将数据类型指定为Trait Playable
// Trait Playable不能作为数据类型
// fn t1() {
// let x: Playable = Audio {
//     name: "telephone.mp3".to_string(),
//     duration: 3.42,
// };
// }
// 这很容易理解，因为一种类型可能实现了很多种Trait，将其实现的其中一种Trait作为数据类型，显然无法代表该类型

// Rust真正支持的用法是：虽然Trait自身不能当作数据类型来使用，但Trait Object可以当作数据类型来使用。
// 因此，可以将实现了Trait A的类型B、C、D当作Trait A的Trait Object来使用。也就是说，
// Trait Object是Rust支持的一种数据类型，它可以有自己的实例数据，就像Struct类型有自己的实例对象一样

// 当类型B实现了Trait A时，类型B的实例对象b可以当作A的Trait Object类型来使用，
// b中保存了作为Trait Object对象的数据指针(指向B类型的实例数据)和行为指针(指向vtable)

// 一定要注意，此时的b被当作A的Trait Object的实例数据，而不再是B的实例对象，而且，
// b的vtable只包含了实现自Trait A的那些方法，因此b只能调用实现于Trait A的方法，
// 而不能调用类型B本身实现的方法和B实现于其他Trait的方法。也就是说，当作哪个
// Trait Object来用，它的vtable中就包含哪个Trait的方法

trait A {
    fn a(&self) {
        println!("from A");
    }
}

trait X {
    fn x(&self) {
        println!("from X");
    }
}

// 类型B同时实现trait A和trait X
// 类型B还定义自己的方法b
struct B {}

impl B {
    fn b(&self) {
        println!("from B");
    }
}
impl A for B {}
impl X for B {}

fn main() {
    // bb是A的Trait Object实例，
    // bb保存了指向类型B实例数据的指针和指向vtable的指针
    let bb: &dyn A = &B {};
    bb.a(); // 正确，bb可调用实现自Trait A的方法a()

    // bb.x(); // 错误，bb不可调用实现自Trait X的方法x()
    // bb.b(); // 错误，bb不可调用自身实现的方法b()
}
