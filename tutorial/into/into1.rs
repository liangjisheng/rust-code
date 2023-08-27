// into和from是Rust语言中两个用于类型转换的函数，它们分别属于Into和From这两个trait
// into函数是Rust语言中的一个转换函数，它属于Into trait。它可以将一个类型转换为另一个类型。
// 实现了From trait的类型会自动获得Into trait的实现，因此通常建议实现From而不是直接实现Into。
// 例如，我们可以很容易地将一个 str 转换为 String

// into和from是Rust语言中两个用于类型转换的函数，它们分别属于Into和From这两个trait。
// From trait允许您定义如何从另一种类型创建自身类型，从而提供了一种非常简单的机制来在多种类型之间进行转换

// Into trait则是From trait的反向操作。也就是说，如果您已经为您的类型实现了 From trait，
// 那么当需要时 Into 会调用它。通常情况下，使用 Into trait时需要指定要转换为的类型，
// 因为编译器大多数时候无法确定这一点。但这只是一个小小的代价，因为我们可以免费获得这个功能

// 在Rust中，类型转换必须是明确和安全的。这意味着不能随意地将一个类型转换为另一个类型，
// 而是必须通过实现特定的trait来定义类型之间的转换关系。

// 例如，要使用into函数进行类型转换，目标类型必须实现From trait，或者源类型必须实现Into trait
// 这样，编译器才能确保类型转换是安全的，并且不会导致未定义行为

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(32); // 函数类似于String::from()（String 转成 &str）
    println!("My Number-num is = {:?}", num);

    let int_0 = 6;
    let num2: Number = int_0.into(); // 函数类似于to_string（&str 转成 String）
    println!("My Number-num2 is = {:?}", num2);

    let my_str = "hello";
    let my_string = String::from(my_str);
    let s1: String = my_str.into();
    println!("{}", s1);
}
// My Number-num is = Number { value: 32 }
// My Number-num2 is = Number { value: 6 }
