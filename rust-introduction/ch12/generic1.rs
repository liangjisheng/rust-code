// impl实现类型的方法时使用泛型

struct Container<T> {
    item: T,
}

// impl后的T是声明泛型T
// Container<T>的T对应Struct类型Container<T>
impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

// Trait使用泛型：
// 表示将某种类型T转换为当前类型
// trait From<T> {
//     fn from(T) -> Self;
// }

// 某数据类型impl实现Trait时使用泛型
use std::fmt::Debug;

trait Eatable {
    fn eat_me(&self);
}

#[derive(Debug)]
struct Food<T>(T);

impl<T: Debug> Eatable for Food<T> {
    fn eat_me(&self) {
        println!("Eating: {:?}", self);
    }
}

impl<T> Food<T> {
    fn new(value: T) -> Food<T> {
        Food(value)
    }
}

// 上面impl时指定了T: Debug，它表示了Food<T>类型的T必须实现了Debug
// 为什么不直接在定义Struct时，将Food定义为struct Food<T: Debug>而是在impl Food时才限制泛型T呢

// 通常，应当尽量不在定义类型时限制泛型的范围，除非确实有必要去限制。这是因为，
// 泛型本就是为了描述更为抽象、更为通用的类型而存在的，限制泛型将使得类型变得更具体化，
// 适用场景也更窄。但是在impl类型时，应当去限制泛型，并且遵守缺失什么功能就添加什么限制的规范，
// 这样可以使得所定义的方法不会过度泛化，也不会过度具体化

// 另一方面，即使定义struct Food<T: Debug>，在impl Food<T>时，
// 也仍然要在impl时指定泛型的限制，否则将编译错误

// 也就是说，如果某个泛型类型有对应的impl，那么在定义类型时指定的泛型限制很可能是多余的
// 但如果没有对应的impl，那么可能有必要在定义泛型类型时加上泛型限制

fn main() {
    let v = Food(0);
    println!("{:?}", v);
    v.eat_me();

    let v1 = Food::new(1);
    println!("{:?}", v1);
    v1.eat_me();

    let v2 = Food::<i32>::new(2);
    println!("{:?}", v2);
}
