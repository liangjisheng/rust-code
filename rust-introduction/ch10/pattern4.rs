// 匹配守卫允许匹配分支添加额外的后置条件：当匹配了某分支的模式后，
// 再检查该分支的守卫后置条件，如果守卫条件也通过，则成功匹配该分支

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let x = 33;
    match x {
        // 先范围匹配，范围匹配成功后，再检查是否是偶数
        // 如果范围匹配没有成功，则不会检查后置条件
        0..=50 if x % 2 == 0 => {
            println!("x in [0, 50], and it is an even");
        }
        0..=50 => println!("x in [0, 50], but it is not an even"),
        _ => (),
    }

    // 后置条件的优先级很低
    // 下面两个分支的写法等价
    // 4 | 5 | 6 if bool_expr => println!("yes"),
    // (4 | 5 | 6) if bool_expr => println!("yes"),

    // 在解构赋值时，如果解构的是一个引用，则被匹配的变量也将被赋值为对应元素的引用
    let t = &(1, 2, 3); // t 是一个引用
    let (t0, t1, t2) = t; // t0,t1,t2的类型都是&i32
    let t0 = t.0; // t0的类型是i32而不是&i32，因为t.0等价于(*t).0
    let t0 = &t.0; // t0的类型是&i32而不是i32，&t.0等价于&(t.0)而非(&t).0

    // 当使用模式匹配语法for i in t进行迭代时
    // 如果t不是一个引用，则t的每一个元素都会 move 给 i
    // 如果t是一个引用，则i将是每一个元素的引用
    // 同理，for i in &mut t和for i in mut t也一样

    // 当match VALUE的VALUE是一个解引用*xyz时(因此，xyz是一个引用)
    // 可能会发生所有权的转移，此时可使用 xyz 或 &*xyz 来代替 *xyz
    // p是一个Person实例的引用
    let p = &Person {
        name: "alice".to_string(),
        age: 23,
    };

    // 使用&*p或p进行匹配，而不是*p
    // 使用*p将报错，因为会转移所有权
    match &*p {
        Person { name, age } => {
            println!("{}, {}", name, age);
        }
        _ => (),
    }

    println!("{:?}", p);
}
