// 还可以在结构体中使用线程局部变量

use std::cell::RefCell;
use std::thread::LocalKey;

struct Foo;
impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}

// 或者通过引用的方式使用它

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(1);
}

struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}

impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}

fn main() {
    Foo::FOO.with(|x| println!("{:?}", x));

    let b = Bar::constructor();
    println!("{:?}", b.foo);
}
