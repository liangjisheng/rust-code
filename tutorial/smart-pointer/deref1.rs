// Deref 强制转换（deref coercions）将实现了 Deref trait 的类型的引用转换为另一种类型的引用。
// 例如，Deref 强制转换可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以
// 返回 &str。Deref 强制转换是 Rust 在函数或方法传参上的一种便利操作，并且只能作用于实现了
// Deref trait 的类型。当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时将自动进行。
// 这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。

// 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref
// 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用 Deref 强制转换并没有运行时损耗

// Deref 强制转换的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 *
// 的引用和解引用。这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码

struct MyBox<T>(T);

use ::std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn print(m: &i32) {
    println!("{}", m);
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    // 使用参数＆b调用print(&b)函数，它是&Box <i32>的引用。 在这种情况下
    // 实现Deref trait，通过Deref强制过程将&Box <i32>转换为&i32
    let b = MyBox::new(5);
    print(&b);

    // 我们调用 hello 并传入 &name，Rust 会自动调用 deref 方法将 &name 转换成 &String 类型
    // 然后再透过 String 的 Deref 实现转换成 &str 类型，最终才能够成功被调用
    let name = MyBox::new(String::from("alice"));
    hello(&name);
}

// 使用Deref Trait覆盖不可变引用上的*运算符，可以使用DerefMut Trait覆盖可变引用上的*运算符

// Rust在以下三种情况下执行Deref强制
// 当T: Deref <Target = U> 其中T和U是不可变引用时，则 &T 转换为 &U 类型
// 当T: DerefMut <Target = U>，其中T和U是可变引用时，则 &mut T 被转换为 &mut U
// 当T: Deref <Target = U>，其中T是可变引用而U是不可变引用，则 &mut T 被转换为 &U

// 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：
// 不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，
// 其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用
// 永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一
// 的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
