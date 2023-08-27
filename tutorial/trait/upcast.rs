use std::fmt::Debug;
use std::mem::transmute;

fn t1() {
    let v = vec![1, 2, 3, 4];
    let a: &Vec<u64> = &v;

    // 转为 trait object
    let b: &dyn Debug = &v;
    println!("a: {}", a as *const _ as usize);
    // 通过 transmute 将 trait object 的引用转为两个 usize，并且验证它们是指向数据与函数虚表的指针
    // Rust 使用 fat pointer（即两个指针） 来表示 trait object 的引用，
    // 分别指向 data 与 vtable，这和 Go 中的 interface 十分类似
    println!("b: {:?}", unsafe { transmute::<_, (usize, usize)>(b) });
}
// a: 140735227204568
// b: (140735227204568, 94484672107880)

// 通过引入一个 AsBase 的 trait 来解决向上转型（upcast）, 将 &dyn SubTrait 转换到 &dyn Base

trait Base {
    fn base(&self) {
        println!("base...");
    }
}

trait AsBase {
    fn as_base(&self) -> &dyn Base;
}

// blanket implementation
impl<T: Base> AsBase for T {
    fn as_base(&self) -> &dyn Base {
        self
    }
}

trait Foo: AsBase {
    fn foo(&self) {
        println!("foo..");
    }
}

#[derive(Debug)]
struct MyStruct;

impl Foo for MyStruct {}
impl Base for MyStruct {}

fn t2() {
    let s = MyStruct;
    let foo: &dyn Foo = &s;
    foo.foo();

    let base: &dyn Base = foo.as_base();
    base.base();
}

fn main() {
    // t1();
    t2();
}
