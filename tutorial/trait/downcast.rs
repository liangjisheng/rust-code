// 向下转型（downcast）
// 向下转型是指把一个 trait object 再转为之前的具体类型，Rust 提供了 Any 这个 trait 来实现这个功能
// 大多数类型都实现了 Any，只有那些包含非 'static 引用的类型没有实现。通过 type_id 就能够在运行时判断类型

use std::any::Any;

trait Greeting {
    fn greeting(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

struct Cat;

impl Greeting for Cat {
    fn greeting(&self) -> &str {
        "Meow!"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let cat = Cat;
    let r = &cat;
    println!("{}", r.greeting());
    println!("{:?}", r.as_any());

    let g: &dyn Greeting = &cat;
    println!("greeting {}", g.greeting());

    // &Cat 类型
    let downcast_cat = g.as_any().downcast_ref::<Cat>().unwrap();
    println!("greeting {}", downcast_cat.greeting());
}

// downcast_ref，其实现为
// pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
//     if self.is::<T>() {
//         unsafe { Some(&*(self as *const dyn Any as *const T)) }
//     } else {
//         None
//     }
// }
