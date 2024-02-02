fn v1() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}", v.len());
    println!("{:?}", v);

    // 迭代向量的值, 循环结束后 v 不可用, v 的所有权发生变更, for 拿走了 v 的所有权
    // 向量本身就实现了迭代器特质，因此可以直接使用 for in 语法来遍历向量
    for i in v {
        println!("{}", i);
    }
    // 修复的方式，就是在使用使用 for in 来迭代向量的一个引用
    // println!("{:?}",v); // 运行出错，因为向量已经不可用

    let mut v1 = Vec::new();
    v1.push(2);
    v1.push(3);
    v1.push(4);

    // 迭代向量的一个引用
    for i in &v1 {
        print!("{} ", i);
    }
    println!("{:?}", v1);

    // 也可以在迭代过程中，修改 Vector 中的元素
    for i in &mut v1 {
        *i += 10
    }

    for v in v1.iter() {
        print!("{} ", v);
    }
    println!();
}

// 使用 Vec::new() 方法创建一个向量的实例，然后在使用 push() 方法添加元素的操作看起来有点复杂。
// 为了使创建向量看起来像创建数组那么简单，Rust 标准库提供了 vec! 用于简化向量的创建。
// 使用 vec! 宏创建向量时，向量的数据类型由第一个元素自动推断出来

fn v2() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    // 向量也是相同类型元素的集合
    // 因此，如果给向量传递了不同数据类型的值则会引发错误 error[E0308]: mismatched types
    // let v = vec![1, 2, 3, "hello"];
    // println!("{:?}", v);

    // 移除索引为 1 的元素并返回
    v.remove(1);
    println!("{:?}", v);

    if v.contains(&3) {
        println!("found 3");
    }
    println!("{:?}", v);
    println!("index 0: {}, index 1: {}", v[0], v[1]);

    // 读取指定位置的元素有两种方式可选：
    // 通过下标索引访问。
    // 使用 get 方法。
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // &v[100] 的访问方式会导致程序无情报错退出，因为发生了数组越界访问。 但是 v.get 就不会，
    // 它在内部做了处理，有值的时候返回 Some(T)，无值的时候返回 None，因此 v.get 的使用方式非常安全。

    // 初始化 vec 的更多方式：
    let v = vec![0; 3]; // 默认值为 0，初始长度为 3
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);

    // 如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 创建动态数组，
    // 这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能
    let mut v: Vec<i32> = Vec::with_capacity(5);
    println!("v.len {}", v.len()); // 0
    println!("v.capacity {}", v.capacity()); // 5

    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    // Vector 常见的一些方法示例
    let mut v = vec![1, 2];
    assert!(!v.is_empty()); // 检查 v 是否为空

    v.insert(2, 3); // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3]
    assert_eq!(v.remove(1), 2); // 移除指定位置的元素并返回, v: [1, 3]
    assert_eq!(v.pop(), Some(3)); // 删除并返回 v 尾部的元素，v: [1]
    assert_eq!(v.pop(), Some(1)); // v: []
    assert_eq!(v.pop(), None); // 记得 pop 方法返回的是 Option 枚举值
    v.clear(); // 清空 v, v: []

    let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
    v.append(&mut v1); // 将 v1 中的所有元素附加到 v 中, v1: []
    v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
    v.retain(|x| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let v2 = m.split_off(1); // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]

    // 当然也可以像数组切片的方式获取 vec 的部分元素
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);
}

// 在 rust 里，实现了两种排序算法，分别为稳定的排序 sort 和 sort_by，以及非稳定排序
// sort_unstable 和 sort_unstable_by。

// 当然，这个所谓的 非稳定 并不是指排序算法本身不稳定，而是指在排序过程中对相等元素的处理
// 方式。在 稳定 排序算法里，对相等的元素，不会对其进行重新排序。而在 不稳定 的算法里则不
// 保证这点。

// 总体而言，非稳定 排序的算法的速度会优于 稳定 排序算法，同时，稳定 排序还会额外分配原数
// 组一半的空间。

fn v3() {
    // 使用 vec::sort 对整数 vector 进行排序。替代方案是使用 vec::sort_unstable
    // 它可以更快，但不保留相等元素的顺序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec);

    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort_unstable();
    // assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 上面的代码编译报错，因为在浮点数当中，存在一个 NAN 的值，这个值无法与其他的浮点
    // 数进行对比，因此，浮点数类型并没有实现全数值可比较 Ord 的特性，而是实现了部分可
    // 比较的特性 PartialOrd

    // 可以使用 vec::sort_by 和 PartialOrd::partial_cmp ，对 f32 或 f64 的 vector 进行排序
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    println!("{:?}", vec);

    vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{:?}", vec);

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}

// 对 Person 结构的 Vector 进行排序，通过属性 name 和 age 的自然顺序（按名称和年龄）
// 为了使 Person 可排序，你需要四个 trait Eq, PartialEq, Ord 和 PartialOrd
// 可以简单地 derive 出这些特征。您还可以使用一个 vec:sort_by 方法，提供自定义比较函数：只按年龄排序

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn v4() {
    let mut people: Vec<Person> = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 自然顺序，排序 people  (名字 和 年龄)
    // people.sort();
    people.sort_unstable();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // 用 年龄 排序
    // people.sort_by(|a, b| b.age.cmp(&a.age));
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}

// 排序需要我们实现 Ord 特性，那么如果我们把我们的结构体实现了该特性，是否就不需要我们自
// 定义对比函数了呢？
// 是，但不完全是，实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性。
// 好消息是，你可以 derive 这些属性：

struct Entity {
    pub name: String,
    pub year: i32,
}

fn v5() {
    // 使用&和[]会返回一个引用，这种方式读取动态数组元素，如果越界了会报panic
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    for i in v.iter() {
        print!("{} ", i);
    }
    println!();

    // get(索引值)方法：返回一个Option<&T>
    // 这种方式读取动态数组元素，如果越界了则返回None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![
        Entity {
            name: String::from("alice"),
            year: 10,
        },
        Entity {
            name: String::from("bob"),
            year: 15,
        },
    ];
    let x = &mut v[0];
    x.year = 11;
    println!("{}", v[0].year);

    let v = vec![1, 2, 3, 4, 5];
    for e in v {
        println!("{:#?}", e);
    }
    // v 移动以后再次读 v, value borrowed here after move
    // println!("{:#?}", v);
}

fn v6() {
    // 不能在同一作用域内同时拥有可变和不可变引用”的规则对Vec是适用的
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // first 为不可变借用
    v.push(6); // push方法会传入一个可变借用，违背借用规则

    // immutable borrow later used here, 注释掉下面这句就可以了
    // 因为 first 不会再使用了
    // println!("The first element is {}", first);
}

// 数组的元素必须类型相同, 如果要存储不同类型的元素，要通过使用枚举类型和特征对象来实现不同类型元素的存储
mod m7 {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    fn show_addr(ip: &IpAddr) {
        println!("{:?}", ip);
    }

    pub fn v7() {
        let v = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        for ip in &v {
            show_addr(ip)
        }
    }
}

mod m8 {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

    pub fn v8() {
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in &v {
            ip.display();
        }
    }

    // 需要注意的是，这里必须手动地指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是
    // 特征 IpAddr 的对象，这样就实现了在数组中存储不同的类型。
    // 在实际使用场景中，特征对象数组要比枚举数组常见很多，主要原因在于特征对象非常灵活，而
    // 编译器对枚举的限制较多，且无法动态增加类型。
}

fn main() {
    // v1();
    v2();
    // v3();
    // v4();
    // v5();
    // v6();
    // m7::v7();
    // m8::v8();
}
