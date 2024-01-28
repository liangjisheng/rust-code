// 所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。
// 仅有一些类型提供了自动实现，比如 std 库中的类型。所有其他类型都必须手动实现。

// fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推导（derive，即自动创建）
// fmt::Debug 的实现。但是 fmt::Display 需要手动实现。

// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式
    // 可以通过手动实现 fmt::Display 来控制显示效果
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 所以 fmt::Debug 确实使这些内容可以打印，但是牺牲了一些美感。
    // Rust 也通过 {:#?} 提供了 “美化打印” 的功能
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
