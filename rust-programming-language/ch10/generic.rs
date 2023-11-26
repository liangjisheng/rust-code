fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn g1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 可以选择为 Point<f32> 实例实现方法，而不是为泛型 Point 实例
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}

// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
// 这个例子的目的是展示一些泛型通过 impl 声明而另一些通过方法定义声明的情况。
// 这里泛型参数 T 和 U 声明于 impl 之后，因为它们与结构体定义相对应。
// 而泛型参数 T1 和 U1 声明于 fn mix_up 之后，因为它们只是相对于方法本身的。
impl<T, U> Point1<T, U> {
    fn mix_up<T1, U1>(self, other: Point1<T1, U1>) -> Point1<T, U1> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn g2() {
    let p = Point { x: 5, y: 4 };
    println!("p.x = {}", p.x());

    let both_integer = Point1 { x: 5, y: 10 };
    let both_float = Point1 { x: 1.0, y: 4.0 };
    let integer_and_float = Point1 { x: 5, y: 4.0 };

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn main() {
    // g1();
    g2();
}

// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
