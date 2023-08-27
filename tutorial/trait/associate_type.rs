// 关联类型在 trait 定义中指定占位符类型
// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，
// 这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者会针对
// 特定的实现在这个类型的位置指定相应的具体类型。如此可以定义一个使用多种类型的
// trait，直到实现此 trait 时都无需知道这些类型具体是什么

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// 其中Item是占位类型，在有类型实现Iterator的时候，需要指定Item的类型
// 一个类型Counter实现Iterator
impl Iterator for Counter {
    type Item = u32; // 指定关联类型

    fn next(&mut self) -> Option<Self::Item> { /* ... */
    }
}

// 通过泛型的形式决定next返回值的类型
pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

// 实际上通过泛型的形式决定next返回值的类型也是可以的，但是如果采用这种形式则在每次调用
// next时都要指定泛型类型是什么。但是有的时候一个trait对于一个类型来说只能有一种实现，
// 此时就需要关联类型而不是泛型。(对于Counter来说只能有一个impl Iterator for Counter，
// 而不应该会返回其他类型的迭代值)
