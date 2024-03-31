use std::collections::HashMap;

#[derive(Debug)]
struct Account {
    id: u32,
}

fn l1() {
    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];

    let mut resolvers = HashMap::new();
    for a in accounts {
        resolvers.entry(a.id).or_insert(Vec::new()).push(a);
    }
    println!("{:?}", resolvers);

    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];
    let mut resolvers = HashMap::new();
    accounts.into_iter().map(|a| {
        resolvers.entry(a.id).or_insert(Vec::new()).push(a);
    });
    // 首先, accounts被拿走所有权后转换成一个迭代器，其次该迭代器通过map方法生成一个新的迭代器，
    // 最后，在此过程中没有以类如collect的消费者适配器收尾
    println!("{:?}", resolvers);
}

// 下面列出三种合理的解决办法
fn l2() {
    // 不再使用迭代器适配器map，改成for_each
    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];
    let mut resolvers = HashMap::new();
    accounts.into_iter().for_each(|a| {
        resolvers.entry(a.id).or_insert(Vec::new()).push(a);
    });
    println!("{:?}", resolvers);

    // 使用消费者适配器collect来收尾，将map产生的迭代器收集成一个集合类型:
    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];
    let resolvers: HashMap<_, _> = accounts.into_iter().map(|a| (a.id, a)).collect();
    println!("{:?}", resolvers);

    // 使用fold，语义表达更强
    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];
    let resolvers = accounts
        .into_iter()
        .fold(HashMap::new(), |mut resolvers, a| {
            resolvers.entry(a.id).or_insert(Vec::new()).push(a);
            resolvers
        });
    println!("{:?}", resolvers);
}

fn l3() {
    let s = "hello world";
    // split 返回迭代器适配器
    let mut words = s.split(" ");
    // count 方法拿走了 words 的所有权, 所以下面不能再次使用 words
    // let n = words.count();

    // 使用 clone
    let n = words.clone().count();
    println!("{:?}", words);

    // 在 Rust 中clone不总是性能低下的代名词，因为clone的行为完全取决于它的具体实现
    // 对于迭代器而言，它其实并不需要持有数据才能进行迭代，事实上它包含一个引用，该引用
    // 指向了保存在堆上的数据，而迭代器自身的结构是保存在栈上。
    // 因此对迭代器的clone仅仅是复制了一份栈上的简单结构，性能非常高效
}

// 我们曾经学习过两个概念：迭代器适配器和消费者适配器，前者用于对迭代器中的元素进行操作，
// 最终生成一个新的迭代器，例如map、filter等方法；而后者用于消费掉迭代器，最终产生一个
// 结果，例如collect方法

fn main() {
    // l1();
    // l2();
    l3();
}
