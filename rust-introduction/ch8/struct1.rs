// Struct就像面向对象的类一样，Rust允许为Struct定义实例方法和关联方法
// 实例方法可被所有实例对象访问调用，关联方法类似于其他语言中的类方法或静态方法

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

// 也可以将方法定义在多个impl Struct_Name {}中
impl Rectangle {
    fn include(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联方法new：构造Rectangle的实例对象
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 所有Struct的实例方法的第一个参数都是self(的不同形式)
// fn f(self)：当obj.f()时，转移obj的所有权，调用f方法之后，obj将无效
// fn f(&self)：当obj.f()时，借用而非转移obj的只读权，方法内部不可修改obj的属性，调用f方法之后，obj依然可用
// fn f(&mut self)：当obj.f()时，借用obj的可写权，方法内部可修改obj的属性，调用f方法之后，obj依然可用

// 定义方法时很少使用第一种形式fn f(self)，因为这会使得调用方法后对象立即消失
// 如果仔细观察的话，会发现方法的第一个参数self(或其他形式)没有指定类型。实际上，在方法的定义中，
// self的类型为Self(首字母大写)。例如，为Rectangle定义方法时，Self类型就是Rectangle类型。
// 因此，下面几种定义方法的方式是等价的
// fn f(self)
// fn f(self: Self)

// fn f(&self)
// fn f(self: &Self)

// fn f(&mut self)
// fn f(self: &mut Self)

// Rust不会自动引用或自动解除引用，但有例外：当使用.运算符和比较操作符(如= > >=)时，
// Rust会自动创建引用和解引用，并且会尽可能地解除多层引用
// (1) 方法调用v.f()会自动解除引用或创建引用
// (2) 属性访问p.name或p.0会自动解除引用
// (3) 比较操作符的两端如果都是引用类型，则自动解除引用
// (4) 能自动解除的引用包括普通引用&x、Box<T>、Rc<T>等

// 当使用ob.something()调用方法时，Rust会根据所调用方法的签名进行推断
// (即根据方法的接收者self参数的形式进行推断)，然后自动为object添加
// &, &mut来创建引用或添加*来自动解除引用，其目的是让obj与方法签名相匹配

// 下面代码是等价的 但第一行看起来简洁的多
// p1.distance(&p2);
// (&p1).distance(&p2);

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{},{}", rect1.area(), rect1.perimeter());

    // 调用关联方法
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(20, 50);
    println!("{}", rect1.area());
    println!("{}", rect2.area());

    // 实际上，实例方法也属于关联方法，也可以采用关联方法的形式去调用
    // 只不过这时需要手动传递第一个self参数。例如
    // 调用Rectangle的area方法，并传递参数 &self
    let v = Rectangle::area(&rect1);
    println!("{}", v);
}
