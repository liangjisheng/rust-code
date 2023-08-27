// 在枚举类中表示泛型的方法诸如 Option 和 Result
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Rust 中的结构体和枚举类都可以实现泛型机制
struct Point<T> {
    x: T,
    y: T,
}

// 结构体与枚举类都可以定义方法，那么方法也应该实现泛型的机制，否则泛型的类将无法被有效的方法操作
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 注意，impl 关键字的后方必须有 <T>，因为它后面的 T 是以之为榜样的。但我们也可以为其中的一种泛型添加方法
// impl Point<f64> {
//     fn x(&self) -> f64 {
//         self.x
//     }
// }

struct Point1<T1, T2> {
    x: T1,
    y: T2,
}

// 方法 mixup 将一个 Point<T, U> 点的 x 与 Point<V, W> 点的 y 融合成一个类型为 Point<T, W> 的新点
impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    println!("x = {}, y = {}", p1.x, p1.y);
    println!("x = {}", p1.x());
    println!("x = {}, y = {}", p2.x, p2.y);

    // 不允许出现类型不匹配的情况
    // let p = Point {x: 1, y: 2.0};
    // x 与 1 绑定时就已经将 T 设定为 i32，所以不允许再出现 f64 的类型。如果我们想让 x 与 y
    // 用不同的数据类型表示，可以使用两个泛型标识符
    let p3 = Point1 { x: 1, y: 2.0 };
    println!("x = {}, y = {}", p3.x, p3.y);
}
