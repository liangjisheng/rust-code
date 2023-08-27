// Rust 语言中的迭代器都要实现标准库中定义的 Iterator 特质。
// Iterator 特质有两个函数必须实现
// 一个是 iter()，用于返回一个 迭代器 对象。迭代器中存储的值，我们称之为 项 ( items ) 。
// 另一个是 next()，用于返回迭代器中的下一个元素。如果已经迭代到集合的末尾（最后一个项后面）则返回 None
// Rust 语言中所有的集合都实现了 Iterator 特质。我们可以简单的使用 iter() 和 next() 方法来完成迭代

fn iter1() {
    let a = [10, 20, 30];

    let mut iter = a.iter(); // 从一个数组中返回迭代器
    println!("{:?}", iter);

    //使用 next() 方法返回迭代器中的下一个元素
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let a = [10, 20, 30];
    let iter = a.iter();
    // for 循环来使用迭代器
    for data in iter {
        print!("{}\t", data);
    }
    println!();
}

// 遍历的迭代器有 4 种：
// 只读遍历但不可重新遍历
// 只读遍历但可以重新遍历
// 可修改遍历但不可重新遍历
// 可修改遍历但不可重入遍历
// 最后一种 可修改遍历但不可重入遍历 感觉没啥大作用。都已经修改元素了但限制遍历，那要怎么访问啊。

// iter()	返回一个只读可重入迭代器，迭代器元素的类型为 &T
// into_iter()	返回一个只读不可重入迭代器，迭代器元素的类型为 T
// iter_mut()	返回一个可修改可重入迭代器，迭代器元素的类型为 &mut T

fn iter2() {
    // iter() 充分体现了 Rust 中 借用 的概念。它返回的迭代器只是一个指向集合元素的引用。
    // 因为只是引用，所以集合保持不变，并且迭代器在遍历之后还可以继续使用

    let names = vec!["alice", "bob", "mike"];
    for name in names.iter() {
        match name {
            &"alice" => println!("我们当中有一个异类!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names); // 迭代之后可以重用集合
}

fn into_iter() {
    // into_iter() 同 iter() 一样返回的是只读迭代，但还是有些不同的，into_iter()
    // 充分运用了 所有权 ownership 的概念。它会把所有迭代的值从集合中移动到一个迭代器对象中。
    // 这样，我们的迭代变量就是一个普通对象而不是对集合元素的引用。在 match 匹配时就不需要引用 & 了
    // iter_into() 之后的集合不可重用

    let names = vec!["alice", "bob", "mike"];
    for name in names.into_iter() {
        match name {
            "alice" => println!("alice abnormal"),
            _ => println!("Hello {}", name),
        }
    }

    // 迭代器之后集合不可再重复使用，因为元素都被拷贝走了
    // println!("{:?}", names);
}

fn iter_mut() {
    // 如果在迭代集合的同时修改集合的元素，则需要使用 iter_mut() 方法代替 iter() 方法。
    // iter_mut() 方法返回的迭代元素是一个 引用类型 或者说是智能指针。我们可以通过对迭代变量 解引用 的方式来重新赋值。
    // 这种重新赋值会修改集合的原元素。
    // iter_mut() 之后的集合是可以重复使用的

    let mut names = vec!["alice", "bob", "mike"];
    for name in names.iter_mut() {
        match name {
            &mut "alice" => {
                *name = "john";
                println!("alice abnormal");
            }
            _ => println!("Hello {}", name),
        }
    }

    // 迭代之后可以重用集合
    println!("{:?}", names);
}

// 消费迭代器指消费迭代器元素的方法，这些方法内部默认都会调用next()方法，
// 从而消费迭代器中的每一个元素。 常用的消费迭代器包括sum、any、collect
// 其它消费迭代器可以在std::iter::Iterator中找到
// 消费迭代器方法是惰性的
fn i1() {
    let v = [1, 2, 3];
    let total: i32 = v.iter().sum();
    println!("total {}", total);

    // any：检查迭代器中是否存在满足给定条件的元素
    let v = [1, 2, 3, 4, 5];
    let result1 = v.iter().any(|&x| x == 2);
    let result2 = v.iter().any(|x| *x == 2);
    println!("res1 {}", result1);
    println!("res2 {}", result2);

    // collect：将迭代器转换为指定的容器类型，即将迭代器中的元素收集到指定的容器中
    let v1 = [1, 2, 3, 4, 5];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
}

// 迭代器适配器的作用是将当前迭代器转化为另一种类型的迭代器，并支持链式调用多个迭代器适配器
fn i2() {
    let v = [1, 2, 3];
    let result: Vec<i32> = v.iter().map(|x| x + 3).collect();

    // take：生成一个仅迭代原迭代器前n个元素的新迭代器，常用于遍历指定数量元素的场景
    let v = [1, 2, 3, 4, 5];
    let result = v.iter().take(3);
    for i in result {
        print!("{} ", i);
    }
    println!();
    // 1 2 3

    // filter：对迭代器中每个元素调用闭包，并生成一个过滤元素的新迭代器，
    // 要求闭包会返回true或false，true则将元素放入新迭代器，否则元素被忽略
    let v = [1, 2, 3];
    let result: Vec<i32> = v.iter().map(|x| x + 3).filter(|x| x % 3 == 0).collect();
    // [6]

    // rev：获取一个与原迭代器迭代顺序反向的迭代器
    let v = [1, 2, 3];
    let r: Vec<_> = v.iter().rev().collect(); // [3,2,1]

    // zip：将两个迭代器压缩成一个新的迭代器，每一次会从两个迭代器中各获取一个元素，
    // 并构成一个元组作为zip迭代器的返回，当其中一个迭代器返回None，则迭代器zip返回None
    let v1 = [1, 2, 3];
    let v2 = [2, 3, 4];
    // (&i32, i32)
    let v: Vec<_> = v1.iter().zip(v2).collect();
    // [(1,2), (2,3), (3,4)]

    // (&i32, &i32)
    let v1: Vec<_> = v1.iter().zip(v2.iter()).collect();
    // [(1,2), (2,3), (3,4)]
    println!("{:?}", v2);
}

fn main() {
    // iter1();
    // iter2();
    // into_iter();
    // iter_mut();

    // i1();
    i2();
}
