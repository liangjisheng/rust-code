// 可以为 trait 中某些签名方法提供默认的行为，这样当某个特定的类型实现 trait 时，可以选择保留或重载签名方法提供的默认行为

pub trait Person {
    fn intro_author(&self) -> String;
    fn profession(&self) -> String;
    fn intro(&self) -> String {
        format!(
            "我的个人简介：{}, 职业 {}",
            self.intro_author(),
            self.profession()
        )
    }
}

pub struct Worker {
    pub name: String,
    pub age: i32,
}

impl Person for Worker {
    fn intro_author(&self) -> String {
        format!("姓名: {} 年龄: {}", self.name, self.age)
    }
    fn profession(&self) -> String {
        format!("打工人")
    }
}

fn main() {
    let w = Worker {
        name: String::from("alice"),
        age: 20,
    };
    println!("{}", w.intro_author());
    println!("{}", w.profession());
    println!("{}", w.intro());
}
