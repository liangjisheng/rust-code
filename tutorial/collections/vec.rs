fn v1() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}", v.len());
    println!("{:?}", v);

    // 迭代向量的值, 循环结束后 v 不可用, v 的所有权发生变更
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

    for v in v1.iter() {
        print!("{} ", v);
    }
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
}

fn v3() {
    // 使用 vec::sort 对整数 vector 进行排序。替代方案是使用 vec::sort_unstable
    // 它可以更快，但不保留相等元素的顺序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec);

    // 可以使用 vec::sort_by 和 PartialOrd::partial_cmp ，对 f32 或 f64 的 vector 进行排序
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    println!("{:?}", vec);

    vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{:?}", vec);
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
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // 用 年龄 排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}

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

fn main() {
    // v1();
    // v2();
    // v3();
    // v4();
    // v5();
    v6();
}
