/*! lib 包是 world_hello 二进制包的依赖包，
里面包含了 computer 等有用模块 */

pub mod art;
pub mod computer;

// 这里是代码注释
// 下面是文档注释

// 文档注释需要位于 lib 类型的包中，例如 src/lib.rs 中
// 文档注释可以使用 markdown语法！例如 # Examples 的标题，以及代码块高亮
// 被注释的对象需要使用 pub 对外可见，记住：文档注释是给用户看的，内部实现细节不应该被暴露出去

// 除了函数、结构体等 Rust 项的注释，你还可以给包和模块添加注释，需要注意的是，这些注释要
// 添加到包、模块的最上方！
// 与之前的任何注释一样，包级别的注释也分为两种：行注释 //! 和块注释 /*! ... */
// 包模块注释，可以让用户从整体的角度理解包的用途，对于用户来说是非常友好的

/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// let a = 5;
/// let b = 6;
/// let answer = comment::add(a, b);
///
/// assert_eq!(11, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = comment::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// 与代码注释一样，文档也有块注释，当注释内容多时，使用块注释可以减少 /// 的使用

/** `add_two` 将指定值加2


```
let arg = 5;
let answer = comment::add_two(arg);

assert_eq!(7, answer);
```
 */
pub fn add_two(x: i32) -> i32 {
    x + 2
}

// 文档注释中的代码跳转
// Rust 在文档注释中还提供了一个非常强大的功能，那就是可以实现对外部项的链接
// 此处的 [Option] 就是一个链接，指向了标准库中的 Option 枚举类型

/// `add_three` 返回一个[`Option`]类型
pub fn add_three(x: i32) -> Option<i32> {
    Some(x + 3)
}

// 再比如，还可以使用路径的方式跳转
use std::sync::mpsc::Receiver;

/// [`Receiver<T>`]   [`std::future`].
///
///  [`std::future::Future`] [`Self::recv()`].
pub struct AsyncReceiver<T> {
    sender: Receiver<T>,
}

impl<T> AsyncReceiver<T> {
    pub async fn recv() -> T {
        unimplemented!()
    }
}

// 除了跳转到标准库，你还可以通过指定具体的路径跳转到自己代码或者其它库的指定项

pub mod a {
    /// `add_one` 返回一个[`Option`]类型
    /// 跳转到[`crate::MySpecialFormatter`]
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }
}

pub struct MySpecialFormatter;

// 如果遇到同名项，可以使用标示类型的方式进行跳转

/// 跳转到结构体  [`Foo`](struct@Foo)
pub struct Bar;

/// 跳转到同名函数 [`Foo`](fn@Foo)
pub struct Foo {}

/// 跳转到同名宏 [`foo!`]
pub fn Foo() {}

#[macro_export]
macro_rules! foo {
    () => {};
}

// 文档搜索别名
// Rust 文档支持搜索功能，我们可以为自己的类型定义几个别名，以实现更好的搜索展现，当别名命
// 中时，搜索结果会被放在第一位：

#[doc(alias = "x")]
#[doc(alias = "big")]
pub struct BigX;

#[doc(alias("y", "big"))]
pub struct BigY;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
