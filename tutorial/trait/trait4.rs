// https://blog.csdn.net/mithril_hermit/article/details/126074868

// 根据trait约束实现方法

// 实现了TraitName1的类型同时也就实现了TraitName2的方法
// impl<T: TraitName1> TraitName2 for T {
// 这里可以实现TraitName2中的方法
// 此时传入的self即有TraitName2的方法也有TraitName1的方法
// }

// 为实现了Display的类型实现 ToString trait
// impl<T: Display> ToString for T {
//     跳过
// }

// Display trait
// 功能描述：以{}格式化输出一个值的内容，主要用于面向用户的输出
// 实现方式：实现Display的fmt(&self, f: &mut Formatter<'_>) -> Result 方法

use std::fmt::{Display, Formatter, Result};

// Debug的例子
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Display的例子
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

fn t1() {
    let origin = Point { x: 0, y: 0 };
    println!("{}", origin); // (0,0)
    println!("{:?}", origin); // Point{ x: 0, y: 0 }
    println!("{:#?}", origin);
    // Point {
    //  x : 0,
    //  y : 0,
    // }
}

// PartialEq trait：
// 实现方式1，实现默认比较逻辑：
// 在需要实现PartialEq的类型上使用 #[derive(PartialEq)]；
// 只有所有字段的值都相等，比较的两个值才相等，只要有任何字段的值不相等则这两个值不相等
// 实现方式2，自定义实现比较逻辑：
// 类型实现 PartialEq 的 fn eq(&self, other: &Self) -> bool 方法

// Eq trait：
// 实现方式：先实现PartialEq，再实现#[derive(Eq)]即可

// 实现Eq的例子
#[derive(PartialEq, Eq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

// 实现 PartialEq 的例子
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

fn t2() {
    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 5,
        format: BookFormat::Paperback,
    };

    assert!(b1 == b2);
    assert!(b1 != b3);
}

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn t3() {
    let person1 = Person {
        id: 1,
        name: String::from("alice"),
        height: 168,
    };
    let person2 = Person {
        id: 2,
        name: String::from("lisi"),
        height: 175,
    };
    let person3 = Person {
        id: 3,
        name: String::from("bob"),
        height: 180,
    };

    assert_eq!(person1 < person2, true);
    assert_eq!(person2 > person3, false);
    assert_eq!(person1.lt(&person2), true);
    assert_eq!(person3.gt(&person2), true);

    let tallest_person = person1.max(person2).max(person3);
    println!("id: {}, name: {}", tallest_person.id, tallest_person.name);
}

// 实现Clone可以使类型获得深复制的能力，即对栈上和堆上的数据一起复制
// 实现Copy的类型具有按位复制其值的能力
// Copy继承自Clone，这意味着要实现Copy的类型，必须实现Clone的clone方法

#[derive(Copy, Clone)]
struct MyStruct1 {}

// 等价于

struct MyStruct;
impl Copy for MyStruct {}
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}

// Rust为数字类型、字符类型、布尔类型、单元值、引用等实现了Copy。
// 对于结构体来说，只有所有字段都实现了Copy，这个结构体才算实现了Copy。
// Copy是一个隐式行为。开发者不能重载Copy行为，它永远是简单的位复制。Copy的隐式行为常发生在执行变量绑定、函数参数传递、函数返回等场景中
// 任何类型都可以实现Clone，开发者可以按需实现clone方法。
// 是否实现Copy可以作为类型是值语义还是引用语义的依据。
// 结构体类型中只要包含了引用语义的类型字段，就实现不了Copy(会报错)

// Default 功能描述：为类型提供获取默认值的方法，通常用于为结构体的字段提供默认值
// 实现方式1，默认实现：#[derive(Default)]，要求结构体的每个字段或枚举都实现了Default的fn default() -> Self;方法
// 实现方式2，自定义实现：实现Default的fn default() -> Self;方法

#[derive(Default, Debug)]
struct MyStruct2 {
    foo: i32,
    bar: f32,
}

// 下面的代码等价于 #[derive(Default)]
/*
impl Default for MyStruct {
    fn default() -> Self {
        MyStruct {foo: 0, bar: 0.0}
    }
}
*/

fn t4() {
    let options1: MyStruct2 = Default::default();
    let options2 = MyStruct2 {
        foo: 7,
        ..Default::default()
    };

    println!("options1: {:?}", options1); // options1: MyStruct {foo:0, bar:0.0}
    println!("options1: {:?}", options2); // options2: MyStruct {foo:7, bar:0.0}
}

fn main() {
    // t1();
    // t2();
    // t3();
    t4();
}
