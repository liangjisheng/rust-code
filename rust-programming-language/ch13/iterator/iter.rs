// 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果

// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait
// pub trait Iterator {
//     type Item;

// fn next(&mut self) -> Option<Self::Item>;

// 此处省略了方法的默认实现
// }

// next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，
// 封装在 Some 中，当迭代器结束时，它返回 None

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // 注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    // 换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
    // 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// Iterator trait 中默认实现的一些方法中有些调用了 next, 这些调用 next 方法的方法被称为 消费适配器
// （consuming adaptors），因为调用它们会消耗迭代器。一个消费适配器的例子是 sum 方法
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），它们允许我们将
// 当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须
// 调用一个消费适配器方法以便获取迭代器适配器调用的结果。

fn iterator_consumer() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // let _ =  v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

// 使用捕获其环境的闭包

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

fn main() {
    let v1 = vec![1, 2, 3];

    // 迭代器被储存在 v1_iter 变量中，而这时没有进行迭代。一旦 for 循环开始使用 v1_iter，
    // 接着迭代器中的每一个元素被用于循环的一次迭代
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    iterator_demonstration();
    iterator_sum();
    iterator_consumer();
    filters_by_size();
}
