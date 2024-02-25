// 为外部类型实现外部特征

// 在之前的章节中，我们有讲过，如果在外部类型上实现外部特征必须使用 newtype 的方式，
// 否则你就得遵循孤儿规则：要为类型 A 实现特征 T，那么 A 或者 T 必须至少有一个在当
// 前的作用范围内。

// 例如，如果想使用 println!("{}", v) 的方式去格式化输出一个动态数组 Vec，以期给
// 用户提供更加清晰可读的内容，那么就需要为 Vec 实现 Display 特征，但是这里有一个问题：
// Vec 类型定义在标准库中，Display 亦然，这时就可以祭出大杀器 newtype 来解决：

use std::fmt;
use std::ops::Add;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn n1() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

fn n2() {
    let d = calculate_distance(Meters(10), Meters(20));
    println!("{}", d);
}

fn main() {
    // n1();
    n2();
}
