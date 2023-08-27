// mod 往往是引入与当前文件同级的文件夹下（兄弟模块下）的文件
// crate 代表引用当前文件同级的文件。为同级文件时，意义和self相同。但如果是同级文件夹，不能混用
// super 代表当前文件的上一级目录（父模块）super后面可以直接接函数。也可以接“*”，表示所有函数
// self 代表当前模块

// Example	Explanation
// mod m {}	Define a module, BK EX REF get definition from inside {}. ↓
// mod m;	Define a module, get definition from m.rs or m/mod.rs. ↓
// a::b	Namespace path EX REF to element b within a (mod, enum, ...).
// ::b	Search b relative to crate root. 🗑️
// crate::b	Search b relative to crate root. '18
// self::b	Search b relative to current module.
// super::b	Search b relative to parent module.
// use a::b;	Use EX REF b directly in this scope without requiring a anymore.
// use a::{b, c};	Same, but bring b and c into scope.
// use a::b as x;	Bring b into scope but name x, like use std::error::Error as E.
// use a::b as _;	Bring b anonymously into scope, useful for traits with conflicting names.
// use a::*;	Bring everything from a into scope.
// pub use a::b;	Bring a::b into scope and reexport from here.

// src_a.rs和src_b.rs相当于分别是a.rs和c.rs与b.rs的代理

pub mod src_a;
pub mod src_b;
pub use self::src_a::*;
pub use crate::src_b::*;

// 内部建立一个 mod.rs，mod.rs 也是一种桥。可以让外部更方面地访问到 d.rs
// 这种情况和外部建立一个与文件夹同名的rs文件有所不同

pub mod src_d;
// 下面都可以运行
// pub use crate::src_d::d;
// pub use self::src_d::d;
pub use src_d::d;

mod e;
mod f;

// error[E0432]: unresolved import crate::xxxx

// 要在main.rs中，引入src所有文件;
// 比如，这里src下有：e.rs; f.rs;
// 否则在其它地方使用 use crate::e
// 感觉好象crate也用对了，但是就是一直出现上述类似的问题

fn main() {
    println!("Hello, world!");
    src_a::a_echo();
    src_b::b_echo();

    src_d::d::d_echo();

    e::e_echo();
    f::f_echo();
}
