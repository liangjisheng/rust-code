// 在 Rust 中，可以为自定义类型实现 Default trait，以便为它们定义默认值
#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    is_male: bool,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::default(),
            age: i32::default(),
            is_male: bool::default(),
        }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}

// 在 Rust 中，可以为泛型类型实现 Default trait，以便为它们定义默认值
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Default> Default for Pair<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

fn main() {
    // 可以使用 Default trait 来获取一个类型的默认值
    let x: i32 = Default::default();
    println!("The default value of i32 is {}", x);

    let s: String = Default::default();
    println!("The default value of String is '{}'", s);

    let p: Point = Default::default();
    println!("The default value of Point is ({}, {})", p.x, p.y);

    // 在 Rust 中，Option 类型是一个枚举类型，它可以表示一个值的存在或不存在。
    // 如果一个变量的类型是 Option 类型，则它的默认值是 None
    let opt: Option<i32> = Default::default();
    println!("The default value of Option<i32> is {:?}", opt);

    let arr: [i32; 3] = Default::default();
    println!("The default value of [i32; 3] is {:?}", arr);

    let tup: (i32, bool, String) = Default::default();
    println!("The default value of (i32, bool, String) is {:?}", tup);

    let p: Person = Default::default();
    println!("The default value of Person is {:?}", p);

    let color: Color = Default::default();
    println!("The default value of Color is {:?}", color);

    let pair: Pair<i32> = Default::default();
    println!("The default value of Pair<i32> is ({}, {})", pair.x, pair.y);
}
