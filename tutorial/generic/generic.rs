// 传递的参数可以是实现了 PartialOrd 特质的任意类型,
// 这里的意思是参数必须是可比较的
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn add<T: std::ops::Add>(a: T, b: T) -> <T>::Output {
    a + b
}

#[derive(Debug, Default)]
struct Point<T1, T2> {
    x: T1,
    y: T2,
}

impl<T1, T2> Point<T1, T2> {
    fn x(&self) -> &T1 {
        &self.x
    }
    fn y(&self) -> &T2 {
        &self.y
    }
}

fn g1() {
    let v1: Vec<i32> = vec![1, 2, 3];
    println!("v1: {:?}", v1);

    let v2: Vec<&str> = vec!["alice", "bob"];
    println!("v2: {:?}", v2);

    let res1 = largest(1, 2);
    let res2 = largest(1.1, 2.1);
    println!("res1: {}, res2: {}", res1, res2);

    let res1 = add(1, 2);
    let res2 = add(1.1, 2.1);
    println!("res1: {}, res2: {}", res1, res2);

    let res1 = Point { x: 1, y: 2 };
    let res2 = Point { x: 1.1, y: 2.1 };
    let res3 = Point { x: 1, y: 2.1 };
    println!("res1: {:?}, res2: {:?}, res3: {:?}", res1, res2, res3);
    println!("res.x: {}, res.y: {}", res1.x(), res1.y());
}

struct Data<T> {
    value: T,
}

fn g2() {
    // 泛型为 i32
    let t: Data<i32> = Data { value: 350 };
    println!("value is: {}", t.value);
    // 泛型为 String
    let t2: Data<String> = Data {
        value: "Tom".to_string(),
    };
    println!("value is: {}", t2.value);
}

use std::fmt::Display;

// 传递的参数可以是实现了 Display 特质的任意类型
fn print_pro<T: Display>(t: T) {
    println!("Inside print_pro generic function:");
    println!("{}", t);
}

fn g3() {
    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hello TutorialsPoint");
}

fn main() {
    // g1();
    // g2();
    g3();
}
