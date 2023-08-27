// 但如果改一下上面示例的需求，不仅要为f64实现上述功能，还要为i32、f32、u8
// 等类型实现上述功能，这时候使用Trait Object就很冗余了，要为每一个数值类型都实现一次
// 使用泛型则可以解决这类因数据类型而导致的冗余问题

use std::ops::Mul;

trait Area<T> {
    fn get_area(&self) -> T;
}

enum Sharp<T> {
    Square(T),
    Rectangle(T, T),
}

impl<T> Area<T> for Sharp<T>
where
    T: Mul<Output = T> + Clone + Copy,
{
    fn get_area(&self) -> T {
        match *self {
            Sharp::Square(a) => a * a,
            Sharp::Rectangle(a, b) => a * b,
        }
    }
}

fn main() {
    let sharps: Vec<Sharp<_>> = vec![Sharp::Square(3.0_f64), Sharp::Rectangle(3.0_f64, 2.0_f64)];
    let area = sharps[0].get_area();
    println!("{}", area);

    let area = sharps[1].get_area();
    println!("{}", area);
}
