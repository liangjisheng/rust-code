// Trait对象可以被看作一种数据类型，它总是以引用的方式被使用，
// 在运行期间，它在栈中保存了具体类型的实例数据和实现自该Trait的方法。
// 泛型不是一种数据类型，它可被看作是数据类型的参数形式或抽象形式，在编译期间会被替换为具体的数据类型
// Trait Object 方式也称为动态分派(dynamic dispatch)，它在程序运行期间动态地决定具体类型
// 而Rust泛型是静态分派，它在编译期间会代码膨胀，将泛型参数转变为使用到的每种具体类型

fn main() {
    let mut sharps: Vec<&dyn Area> = vec![];
    sharps.push(&Square(3.0));
    sharps.push(&Rectangle(3.0, 2.0));
    println!("{}", sharps[0].get_area());
    println!("{}", sharps[1].get_area());
}

trait Area {
    fn get_area(&self) -> f64;
}

struct Square(f64);
struct Rectangle(f64, f64);

impl Area for Square {
    fn get_area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        self.0 * self.1
    }
}
