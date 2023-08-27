// Unpin 是个标记trait( marker trait )，定义如下
// pub auto trait Unpin {}

// Pin 可以防止一个类型在内存中被移动，而 Unpin 相反，Unpin 则相当于
// 声明的数据结构是可以在内存中安全的移动的，它的作用类似于 Send / Sync
// 通过类型约束来告诉编译器哪些行为是合法的、哪些不是合法的。
// 在 Rust 中，绝大多数数据结构都是可以移动的，所以它们都自动实现了 Unpin
// 即便这些结构被 Pin 住，它们依旧可以进行移动

use std::mem;
use std::pin::Pin;

fn main() {
    let mut string = "this".to_string();
    let mut pinned_string = Pin::new(&mut string);

    // We need a mutable reference to call `mem::replace`.
    // We can obtain such a reference by (implicitly) invoking `Pin::deref_mut`,
    // but that is only possible because `String` implements `Unpin`.
    mem::replace(&mut *pinned_string, "other".to_string());
    println!("{}", string); // other
}

// 当希望一个数据结构不能被移动，可以使用 !Unpin。在 Rust 里，实现了 !Unpin的
// 除了内部结构（比如 Future），主要就是 PhantomPinned，所以如果希望数据结构
// 不能被移动，可以为其添加 PhantomPinned 字段来隐式声明 !Unpin

// pub struct PhantomPinned;
// impl !Unpin for PhantomPinned {}
