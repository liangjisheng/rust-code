// Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// 传递的参数可以是实现了 PartialOrd 特质的任意类型,
// 这里的意思是参数必须是可比较的
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// 上面代码的 T 就是泛型参数，实际上在 Rust 中，泛型参数的名称你可以任意起，但是出于惯例，
// 我们都用 T ( T 是 type 的首字母)来作为首选，这个名称越短越好，除非需要表达含义，否则
// 一个字母是最完美的。
// 使用泛型参数，有一个先决条件，必需在使用前对其进行声明

fn add<T: std::ops::Add>(a: T, b: T) -> <T>::Output {
    a + b
}

#[derive(Debug, Default)]
struct Point<T1, T2> {
    x: T1,
    y: T2,
}

// 如果想让 x 和 y 既能类型相同，又能类型不同，就需要使用不同的泛型参数
// 切记，所有的泛型参数都要提前声明：Point<T1,T2>

impl<T1, T2> Point<T1, T2> {
    // 方法上也可以使用泛型
    // 使用泛型参数前，依然需要提前声明：impl<T>，只有提前声明了，我们才能在
    // Point<T>中使用它，这样 Rust 就知道 Point 的尖括号中的类型是泛型而
    // 不是具体类型。需要注意的是，这里的 Point<T> 不再是泛型声明，而是一个
    // 完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point
    fn x(&self) -> &T1 {
        &self.x
    }
    fn y(&self) -> &T2 {
        &self.y
    }

    // 除了结构体中的泛型参数，我们还能在该结构体的方法中定义额外的泛型参数，就
    // 跟泛型函数一样
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T1, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 对于 Point<T1, T2> 类型，你不仅能定义基于 T 的方法，还能针对特定的具体类型，
// 进行方法定义, 这段代码意味着 Point<f64, f64> 类型会有一个方法
// distance_from_origin，而其他 T 不是 f64 类型的 Point<T1, T2> 实例则没
// 有定义此方法。这样我们就能针对特定的泛型类型实现某个特定的方法，对于其它泛型类型
// 则没有定义该方法。
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
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
    println!("p2 distance {}", res2.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
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

// const 泛型（Rust 1.51 版本引入的重要特性）, 针对值的泛型
// 在之前的泛型中，可以抽象为一句话：针对类型实现的泛型，所有的泛型都是为了抽象不同的类型

// 打印任意长度的数组, arr 参数的类型为数组切片
fn display_array1(arr: &[i32]) {
    println!("{:?}", arr);
}

fn d1() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array1(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array1(&arr);
}

// 将参数泛型化
fn display_array2<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn d2() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array2(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array2(&arr);
}

// 有了 const 泛型，也就是针对值的泛型，正好可以用于处理数组长度的问题
fn display_array3<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn d3() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array3(arr);

    let arr: [i32; 2] = [1, 2];
    display_array3(arr);
}

// 如上所示，我们定义了一个类型为 [T; N] 的数组，其中 T 是一个基于类型的泛型参数，
// 这个和之前讲的泛型没有区别，而重点在于 N 这个泛型参数，它是一个基于值的泛型参数！
// 因为它用来替代的是数组的长度。

// N 就是 const 泛型，定义的语法是 const N: usize，表示 const 泛型 N ，它基于
// 的值类型是 usize。

fn main() {
    // g1();
    // g2();
    // g3();

    // d1();
    // d2();
    // d3();
}
