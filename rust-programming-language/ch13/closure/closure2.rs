// FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。
// 一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
// FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。
// Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。
// 这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。

// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//         where
//             F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key 被定义为接收一个 FnMut 闭包的原因是它会多次调用这个闭包：
    // 每个 slice 中的元素调用一次。闭包 |r| r.width 不捕获、修改或将任何
    // 东西移出它的环境，所以它满足 trait bound 的要求。

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        // 闭包捕获了 value 然后通过转移 value 的所有权的方式将其移出闭包给到
        // sort_operations vector。这个闭包可以被调用一次，尝试再次调用它将报错。
        // 因为这时 value 已经不在闭包的环境中，无法被再次放到 sort_operations 中！
        // 因而，这个闭包只实现了 FnOnce。由于要求闭包必须实现 FnMut, 因此尝试编译
        // 这个代码将得到报错：value 不能被移出闭包
        // sort_operations.push(value);

        num_sort_operations += 1;
        r.width
    });

    println!("{}", num_sort_operations);
    println!("{:#?}", list);
}
