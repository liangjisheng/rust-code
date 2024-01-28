// trait 就是把方法的签名放在一起，定义实现某一目的所必须的一组行为
// 与其它语言的接口（Interface）类似，也还有些区别
// 虽然 Rust 语言没有接口的概念，但 特质 Traits 实打实的就是接口啊
// 如果要为某个结构体 （ struct ）实现某个特质，需要使用 impl for 语句

// 从语法格式来看，特征可以包含具体方法（带实现的方法）或抽象方法（没有具体实现的方法）
// 如果想让某个方法的定义在实现了特质的结构体所共享，那么推荐使用具体方法。
// 如果想让某个方法的定义由实现了特质的结构体自己定义，那么推荐使用抽象方法。
// 即使某个方法是具体方法，结构体也可以对该方法进行重写

// Trait Bounds 适用于复杂的场景，例如方法参数中定义多个实现了 trait 的类型
// pub fn print_intro(item1: impl Person, item2: impl Person) {}
// Trait Bounds 与范型参数声明在一起，改写上面的例子如下所示
// pub fn print_intro<T: Person>(item1: T, item2: T) {}

pub trait Person {
    fn intro(&self) -> String;
}

// 实现 trait 定义的行为
pub struct Worker {
    pub name: String,
    pub age: i32,
}

impl Person for Worker {
    fn intro(&self) -> String {
        format!("My name is {}, age {}, is a worker", self.name, self.age)
    }
}

// 把 trait 做为参数
pub fn print_intro(item: impl Person) {
    println!("pub fn print_intro {}", item.intro());
}

// trait 做为返回值
fn returns_intro() -> impl Person {
    Worker {
        name: String::from("ljs"),
        age: 20,
    }
}

fn t1() {
    let w = Worker {
        name: String::from("ljs"),
        age: 20,
    };
    println!("{}", w.intro());

    print_intro(w);
    print_intro(returns_intro());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn t2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn main() {
    // t1();
    t2();
}
