// Deref <T> trait用于自定义解除引用运算符(*)的行为
// Box <T>指针可用作引用

// 实现 Deref Trait
// deref 方法借用self并返回对内部数据的引用

struct MyBox<T> {
    a: T,
}

use std::ops::Deref;

// type Target = T; 是Deref trait的关联类型。关联类型用于声明泛型类型参数
// 如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
// 在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.a
    }
}

fn main() {
    let a = 11;
    let b = Box::new(a);
    println!("Value of *b is {}", *b);

    let c = MyBox { a: 10 };
    // 调用 deref 手动解引用, 或者直接 *c 自动解引用
    println!("{}", *(c.deref()));
    println!("{}", *c);
}
