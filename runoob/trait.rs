// 特性（trait）概念接近于 Java 中的接口（Interface）
// Descriptive 规定了实现者必需有 describe(&self) -> String 方法
// Rust 同一个类可以实现多个特性，每个 impl 块只能实现一个
trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

// 这是特性与接口的不同点：接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，
// 因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法

trait Descriptive1 {
    fn describe1(&self) -> String {
        String::from("[Object]")
    }
}

impl Descriptive1 for Person {
    // 注释掉使用默认方法
    // fn describe1(&self) -> String {
    //     format!("{} {}", self.name, self.age)
    // }
}

fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24,
    };
    println!("{}", cali.describe());
    println!("{}", cali.describe1());
}
