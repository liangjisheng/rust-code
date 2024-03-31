use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
struct MyType {
    data: Vec<u32>,
}

// 自定义 Clone 方法来实现特定的克隆行为
impl MyType {
    fn custom_clone(&self) -> MyType {
        MyType {
            data: self.data.iter().map(|x| x + 1).collect(),
        }
    }
}

fn c1() {
    let a = 5;
    let b = a.clone();
    println!("a = {}, b = {}", a, b);

    let a = String::from("hello");
    let b = a.clone();
    println!("a = {}, b = {}", a, b);

    let a = [1, 2, 3];
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let a = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let a = Color::Red;
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let a = vec![1, 2, 3];
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let mut a = HashMap::new();
    a.insert("one", 1);
    a.insert("two", 2);
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let a = MyType {
        data: vec![1, 2, 3],
    };
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);

    let a = MyType {
        data: vec![1, 2, 3],
    };
    let b = a.custom_clone();
    println!("a = {:?}, b = {:?}", a, b);

    // 使用 clone 方法来克隆一个闭包
    let a = |x| x + 1;
    let b = a.clone();
    println!("{}", a(1));
    println!("{}", b(2));
}

fn c2() {
    // 克隆一个迭代器
    let a = vec![1, 2, 3].into_iter();
    let b = a.clone();

    for x in a {
        println!("{}", x);
    }

    for x in b {
        println!("{}", x);
    }

    // 克隆一个 Rc
    let a = Rc::new(5);
    let b = a.clone();
    println!("a = {}, b = {}", a, b);
}

fn main() {
    // c1();
    c2();
}
