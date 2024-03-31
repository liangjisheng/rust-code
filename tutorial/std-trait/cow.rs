// https://juejin.cn/post/7221969436885631033

// Cow 是 Rust 语言中的一个特殊类型，全称为 Clone-On-Write，即在写入时进行克隆操作。
// Cow 类型可以用来避免不必要的内存分配和复制操作，从而提高程序的性能和效率。Cow 特征
// 通常用于处理需要多次读取和少量修改的数据结构，比如字符串和向量等。

// Cow 特征是 Rust 语言中的一个标准库特性，用于处理读写分离的数据结构。Cow 类型有两种形式：

// Cow::Borrowed(&'a T): 表示一个不可变的引用，可以用于读取数据；
// Cow::Owned(T): 表示一个可变的数据，可以用于修改数据。

// Cow 类型的克隆操作是惰性的，只有在修改数据时才会进行克隆操作。这种惰性的克隆操作可以避免
// 不必要的内存分配和复制操作，从而提高程序的性能和效率。

use std::borrow::Cow;
use std::clone::Clone;

#[derive(Clone)]
struct Animal<'a> {
    name: Cow<'a, str>,
    age: u32,
    species: Cow<'a, str>,
}

fn c1() {
    let animal = Animal {
        name: Cow::Borrowed("Tom"),
        age: 3,
        species: Cow::Borrowed("Cat"),
    };

    println!("Name: {}", animal.name);
    println!("Species: {}", animal.species);
}

// 在修改 Animal 结构中的 name 和 species 字段时，我们可以使用 Cow::Owned
// 类型来避免不必要的内存分配和复制操作。具体来说，我们可以使用 Cow::Owned 类型
// 来克隆数据，并进行修改操作。

fn c2() {
    let mut animal = Animal {
        name: Cow::Borrowed("Tom"),
        age: 3,
        species: Cow::Borrowed("Cat"),
    };

    animal.name.to_mut().push_str("mycat");
    animal.species = Cow::Owned("Lion".to_string());

    println!("Name: {}", animal.name);
    println!("Species: {}", animal.species);
    // Output
    // Name: Tommycat
    // Species: Lion
}

// Cow::into_owned 方法可以将 Cow 类型转换为 Owned 类型。具体来说，它会在需要
// 修改数据时进行克隆操作，并返回一个可变的数据

fn c3() {
    let animal = Animal {
        name: Cow::Borrowed("Tom"),
        age: 3,
        species: Cow::Borrowed("Cat"),
    };

    let mut name = animal.name.into_owned();
    name.push_str("mycat");

    let mut species = animal.species.into_owned();
    species = "Lion".to_string();

    let animal2 = Animal {
        name: Cow::Owned(name),
        age: 4,
        species: Cow::Owned(species),
    };

    println!("Name: {}", animal2.name);
    println!("Species: {}", animal2.species);

    // 所有权被转移，不能再次使用
    // println!("Name: {}", animal.name);
    // println!("Species: {}", animal.species);
}
//  输出结果：
// Name: Tommycat
// Species: Lion

// Cow::from 方法可以将一个不可变的引用或可变的数据转换为 Cow 类型。具体来说，
// 它会根据数据类型的不同，返回一个 Cow::Borrowed 或 Cow::Owned 类型

fn c4() {
    let name = "Tom".to_string();
    let species = "Cat".to_string();

    let animal = Animal {
        name: Cow::from(&name),
        age: 3,
        species: Cow::from(species),
    };

    println!("Name: {}", animal.name);
    println!("Species: {}", animal.species);
}
//  输出结果：
// Name: Tom
// Species: Cat

fn main() {
    // c1();
    // c2();
    // c3();
    c4();
}

// 尽量使用 Cow::Borrowed 类型来读取数据，避免不必要的内存分配和复制操作；
// 尽量使用 Cow::Owned 类型来修改数据，避免不必要的内存分配和复制操作；
// 在需要使用 Cow 类型时，优先考虑使用 Cow::from 方法来构造 Cow 类型；
// 在需要修改数据时，优先考虑使用 Cow::into_owned 方法或 Cow::into_owned
// 方法来转换 Cow 类型为 Owned 类型；
// 在定义 Cow 类型时，需要使用泛型参数来指定数据类型，避免类型不匹配的错误
