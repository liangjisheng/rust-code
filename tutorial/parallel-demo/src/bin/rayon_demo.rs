// rayon crate，这是一个 Rust 程序设计语言的数据并行库。rayon 为任何并行
// 可迭代的数据类型提供 par_iter_mut 方法。这是一个类迭代器的链，可以对链内
// 的数据并行计算。

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn rayon_demo1() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);

    // 这个实例示范如何使用 rayon::any 和 rayon::all 方法，这两个方法是分别与
    // std::any 和 std::all 相对应的并行方法。rayon::any 并行检查迭代器的任意
    // 元素是否与断言匹配，并在找到一个匹配的元素时就返回。rayon::all 并行检查迭代
    // 器的所有元素是否与断言匹配，并在找到不匹配的元素时立即返回。
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));

    // 使用给定断言并行搜索项
    // 使用 rayon::find_any 和 par_iter 并行搜索 vector 集合，以查找满足指定闭包中
    // 的断言的元素。
    // 如果有多个元素满足 rayon::find_any 闭包参数中定义的断言，rayon 将返回搜索发现的
    // 第一个元素，但不一定是 vector 集合的第一个元素。
    // 请注意，实例中闭包的参数是对引用的引用（&&x）
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));

    // 对 vector 并行排序
    // 通过 par_iter_mut().for_each 并行对 vector 填充随机值
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        let v: Vec<u8> = (0..5).map(|_| rng.sample(&Alphanumeric)).collect();
        *p = String::from_utf8(v).unwrap();
    });
    vec.par_sort_unstable();
    println!("{:?}", &vec[0..5]);
}

// 此实例使用 rayon::filter、rayon::map，以及 rayon::reduce 计算 Person
// 对象中年龄超过 30 岁的那些人的平均年龄。
// rayon::filter 过滤集合中满足给定断言的元素。rayon::map 对每个元素执行一次
// 计算，创建一个新的迭代；然后，基于前一次的 reduce 计算结果和当前元素一起，
// rayon::reduce 执行新的计算。也可以查看 rayon::sum，它与本实例中的 reduce
// 计算具有相同的结果。

struct Person {
    age: u32,
}

fn rayon_demo2() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}

fn main() {
    // rayon_demo1();
    rayon_demo2();
}
