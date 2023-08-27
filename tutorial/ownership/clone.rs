// rust 的类型可以实现Drop trait，也可以不实现Drop trait。但是对于实现了Copy trait
// 的类型，不能实现Drop trait。也就是说Copy和Drop两个 trait 对同一个类型只能有一个

// 变量在离开作用范围时，编译器会自动销毁变量，如果变量类型有Drop trait，就先调用
// Drop::drop方法做资源清理，一般会回收heap内存等资源，然后再收回变量所占用的
// stack 内存。如果变量没有Drop trait，那就只收回 stack 内存。

// 正是由于在Drop::drop方法会做资源清理，所以Copy和Drop trait 只能二选一。如果类型实现了
// Copy trait，在 copy 语义中并不会调用Clone::clone方法，不会做 deep copy，那就会出现
// 两个变量同时拥有一个资源（比如说是 heap 内存等），在这两个变量离开作用范围时，会分别调用
// Drop::drop方法释放资源，这就会出现 double free 错误

/*
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

pub trait Copy: Clone {
    // Empty.
}
*/

// Clone是Copy的 super trait，一个类型要实现Copy就必须先实现Clone。
// 再留意看，Copy trait 中没有任何方法，所以在 copy 语义中不可以调用用户自定义的资源复制代码
// 也就是不可以做 deep copy。copy 语义就是变量在stack内存的按位复制，没有其他任何多余的操作。
// Clone中有 clone 方法，用户可以对类型做自定义的资源复制，这就可以做 deep copy。在 clone
// 语义中，类型的Clone::clone方法会被调用，程序员在Clone::clone方法中做资源复制，同时在
// Clone::clone方法返回时，变量的 stack 内存也会被按照位复制一份，生成一个完整的新实例

// Clone trait，对于任何自定义类型都可以实现。Copy trait只有自定义类型中的field全部实现了
// Copy trait，才可以实现Copy trait

// 也可以通过#[derive(Copy, Clone)]方式实现

struct S1 {
    i: i32,
    u: usize,
}

impl Copy for S1 {}

impl Clone for S1 {
    fn clone(&self) -> Self {
        // 此处是S1的copy语义调用。
        // 正是i32和usize的Copy trait，才有了S1的Copy trait。
        *self
    }
}

enum E1 {
    Text,
    Digit,
}

// 在这种情况下，不能通过#[derive(Clone)]自动实现S2类型的 Clone trait。只有类型中的所有
// field 都有Clone，才可以通过#[derive(Clone)]自动实现 Clone trait

struct S2 {
    u: usize,
    e: E1,
    s: String,
}

impl Clone for S2 {
    fn clone(&self) -> Self {
        // 生成新的E1实例
        let e = match self.e {
            E1::Text => E1::Text,
            E1::Digit => E1::Digit,
        };
        Self {
            u: self.u,
            e,
            s: self.s.clone(),
        }
    }
}

fn main() {
    let s1 = S1 { i: 0, u: 0 };
    let s2 = s1;

    // S1 实现了 clone, 所以 s1 依然可以使用
    println!("{}, {}", s1.i, s1.u);
    println!("{}, {}", s2.i, s2.u);

    let v1 = S2 {
        u: 0,
        e: E1::Digit,
        s: "hello".to_string(),
    };
    let v2 = v1.clone();

    println!("{}, {}", v1.u, v1.s);
    println!("{}, {}", v2.u, v2.s);
}
