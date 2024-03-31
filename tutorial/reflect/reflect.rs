// https://juejin.cn/post/7226525442177761317

// Rust语言的反射机制指的是在程序运行时获取类型信息、变量信息等的能力。Rust语言中的反射机制主要通过Any实现
// std::any::Any trait
// Any trait是所有类型的超级trait，它定义了一些通用的方法，可以对任意类型的值进行操作

use std::alloc::alloc;
use std::alloc::Layout;
use std::any::Any;
use std::any::TypeId;
use std::mem::{size_of, transmute};

fn a1() {
    let a = 1;
    let b = "hello";
    let c = true;

    println!("a's type id: {:?}", a.type_id());
    println!("b's type id: {:?}", b.type_id());
    println!("c's type id: {:?}", c.type_id());
    // 可以看到，每个类型都有一个唯一的类型ID，可以用来判断两个值的类型是否相同
}
// 输出结果为：
// a's type id: TypeId { t: 115387559057565692143404304070439989267 }
// b's type id: TypeId { t: 94774005424224989474030617523037649708 }
// c's type id: TypeId { t: 324675245860759320943513204350442872190 }

fn is_string(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

fn a2() {
    is_string(&0);
    is_string(&"alice".to_string());
}
// Not a string...
// It's a string!

// 可以使用type_name方法获取一个类型的名称
fn a3() {
    let a = 1;
    let b = "hello";
    let c = true;

    println!("a's type name: {:?}", std::any::type_name::<i32>());
    println!("b's type name: {:?}", std::any::type_name::<&str>());
    println!("c's type name: {:?}", std::any::type_name::<bool>());
}
//    输出结果为：
// a's type name: "i32"
// b's type name: "&str"
// c's type name: "bool"

// 在Rust语言中，在某些场景下，需要在运行时才能确定变量的具体类型。在 Rust 中可以使用
// 反射来进行类型检查。具体来说，可以通过Any trait将一个值转换为&Any类型的引用，然后
// 使用TypeId获取该值的类型信息

fn print_type<T: Any>(val: &T) {
    let v_any = val as &dyn Any;
    if let Some(_) = v_any.downcast_ref::<Vec<i32>>() {
        println!("Type: Vec<i32>");
    } else if let Some(_) = v_any.downcast_ref::<Vec<&str>>() {
        println!("Type: Vec<&str>");
    } else {
        println!("Unknown Type");
    }
}

fn a4() {
    let x = vec![1, 2, 3];
    let y = vec!["a", "b", "c"];
    print_type(&x);
    print_type(&y);
}
//  输出结果为：
// Type: Vec<i32>
// Type: Vec<&str>

// 在Rust语言中，可以使用反射机制动态调用函数。具体来说，可以使用std::mem::transmute
// 函数将函数指针转换为一个通用的函数指针，然后使用该指针调用函数。例如，可以定义一个函数
// 指针类型 FnPtr，然后将其转换为一个通用的函数指针类型*const u8，最后使用
// std::mem::transmute 函数将其转换为一个具体的函数指针类型，然后调用该函数

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn a5() {
    let add_ptr = add as *const u8;
    let add_fn: fn(i32, i32) -> i32 = unsafe { transmute(add_ptr) };

    let result = add_fn(1, 2);
    println!("result: {}", result);
}
//  输出结果为：
//  result: 3

// 在Rust语言中，可以使用反射机制动态创建对象。具体来说，可以使用 std::mem::size_of
// 函数获取一个类型的大小，然后使用std::alloc::alloc函数在堆上分配一块内存，最后使用
// std::mem::transmute 函数将该内存转换为一个具体的对象。例如，可以定义一个结构体
// Person，然后使用反射机制动态创建该结构体的实例

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn a6() {
    let size = size_of::<Person>();
    let ptr = unsafe { alloc(Layout::from_size_align(size, 1024).unwrap()) };
    let person: &mut Person = unsafe { transmute(ptr) };

    person.name = "Alice".to_string();
    person.age = 20;

    println!("person: {:?}", person);
}
//  输出结果为：
//  person: Person { name: "Alice", age: 20 }

fn main() {
    // a1();
    // a2();
    // a3();
    // a4();
    // a5();
    a6();
}

// bevy_reflect 是一个Rust语言的工具库，提供了元编程（meta-programming）
// 中非常有用的反射（reflection）功能。反射是指在程序运行时，能够动态地获取
// 一个对象的各种信息，例如类型、结构体字段等。bevy_reflect 提供的反射功能
// 可以让我们更加方便地读取和修改对象的属性，为开发高效、灵活的程序提供了支持
