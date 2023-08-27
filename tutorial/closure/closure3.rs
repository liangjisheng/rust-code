// https://juejin.cn/post/7206237639325827128

// 与闭包关联的是三个 trait 的定义，分别是FnOnce、FnMut和Fn，定义如下

/*
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
*/

// 如果闭包只是对捕获变量的非修改操作，闭包捕获的是&T类型，闭包按照Fn trait 方式执行，闭包可以重复多次执行
// 如果闭包对捕获变量有修改操作，闭包捕获的是&mut T类型，闭包按照FnMut trait 方式执行，闭包可以重复多次执行
// 如果闭包会消耗掉捕获的变量，变量被 move 进闭包，闭包按照FnOnce trait 方式执行，闭包只能执行一次

// 1. 类型实现了Copy，闭包中是&T操作
// f 闭包对 i 变量，没有修改操作，此处捕获到的是&i，所以 f 就是按照Fn trait 方式执行，可以多次执行f
fn test_fn_i8() {
    let mut i = 1_i8;
    let f = || i + 1;

    // f闭包对i是immutable borrowed，是Fn trait
    let v = f();

    // f闭包中只是immutable borrowed，此处可以再做borrowed。
    dbg!(&i);

    // f可以调用多次
    let v2 = f();

    // 此时，f闭包生命周期已经结束，i已经没有borrowed了，所以此处可以mutable borrowed。
    i += 10;

    assert_eq!(2, v);
    assert_eq!(2, v2);
    assert_eq!(11, i);
}

// 2. 类型实现了Copy，闭包中是&mut T操作
// f 闭包对 i 变量，有修改操作，此处捕获到的是&mut i，所以f就是按照
// FnMut trait 方式执行，注意 f 本身也是mut，可以多次执行 f
fn test_fn_mut_i8() {
    let mut i = 1_i8;
    let mut f = || {
        i += 1;
        i
    };

    // f闭包对i是mutable borrowed，是FnMut trait
    let v = f();

    // i已经被 f 闭包 mutable borrowed，就不能再borrowed了
    // immutable borrow occurs here
    // dbg!(&i);

    // f可以调用多次
    let v2 = f();

    // f 闭包生命周期已经结束，可以 immutable borrow
    dbg!(&i);

    // 此时，f闭包生命周期已经结束，i没有mutable borrowed了，所以此处可以mutable borrowed。
    i += 10;

    assert_eq!(2, v);
    assert_eq!(3, v2);
    assert_eq!(13, i);
}

// 3. 类型实现了Copy，闭包使用move关键字，闭包中是&mut T操作
// f 闭包对i变量，有修改操作，并且使用了move关键字。由于i8实现了Copy trait，
// 此处 i 会 copy 一个新实例，并将新实例 move 到闭包中，在闭包中的实际是一个
// 新的i8变量。f 就是按照FnMut trait 方式执行，注意 f 本身也是mut，可以多次执行 f
// 重点说明，此处move关键字的使用，强制 copy 一个新的变量，将新变量move进闭包
fn test_fn_mut_i8_move() {
    let mut i = 1_i8;
    let mut f = move || {
        i += 1;
        i
    };

    // i8有Copy trait，f闭包中是move进去的新实例，新实例不会被消耗，是FnMut trait
    let v = f();

    // i8有Copy trait，f闭包中是move进去的新实例，i没有borrowed，所以此处可以mutable borrowed。
    i += 10;

    // f可以调用多次
    let v2 = f();

    assert_eq!(2, v);
    assert_eq!(3, v2);
    assert_eq!(11, i);
}

// 4. 类型没有实现Copy，闭包中是&T操作
// f 闭包对 s 变量，没有修改操作，此处捕获到的是&s，f 按照Fn trait 方式执行，可以多次执行 f
fn test_fn_string() {
    let mut s = "Hello".to_owned();
    let f = || -> String {
        dbg!(&s);
        "world".to_owned()
    };

    // f闭包对s是immutable borrowed，是Fn trait
    let v = f();

    // f闭包中是immutable borrowed，此处是第二个immutable borrowed。
    dbg!(&s);

    // f可以调用多次
    let v2 = f();

    // f闭包生命周期结束，s已经没有borrowed，所以此处可以mutable borrowed
    s += " moto";

    assert_eq!("world", &v);
    assert_eq!("world", &v2);
    assert_eq!("Hello moto", &s);
}

// 5. 类型没有实现Copy，闭包中是&mut T操作
// f 闭包对 s 变量，调用push_str(&mut self, &str)方法修改，此处捕获到的是&mut s
// f是按照FnMut trait 方式执行，注意 f 本身是mut，f 可以多次执行 f
fn test_fn_mut_string() {
    let mut s = "Hello".to_owned();
    let mut f = || -> String {
        s.push_str(" world");
        s.clone()
    };

    // f闭包对s是mutable borrowed，是FnMut trait
    let v = f();

    // s是mutable borrowed，此处不能再borrowed。
    // dbg!(&s);

    // f可以多次调用
    let v2 = f();

    // f闭包生命周期结束，s已经没有borrowed，所以此处可以mutable borrowed
    s += " moto";

    assert_eq!("Hello world", &v);
    assert_eq!("Hello world world", &v2);
    assert_eq!("Hello world world moto", &s);
}

// 6. 类型没有实现Copy，闭包使用move关键字，闭包中是&mut T操作
// f 闭包对 s 变量，调用push_str(&mut self, &str)方法修改，闭包使用move关键字
// s 被move 进闭包，s 没有被消耗，f 是按照FnMut trait 方式执行，注意f本身是mut，f 可以多次执行
fn test_fn_mut_move_string() {
    let mut s = "Hello".to_owned();
    let mut f = move || -> String {
        s.push_str(" world");
        s.clone()
    };

    // s被move进f闭包中，s没有被消耗，是FnMut trait
    let v = f();

    // s被move进闭包，s不能被borrowed
    // dbg!(&s);

    // f可以多次调用
    let v2 = f();

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例
    s = "moto".to_owned();

    assert_eq!("Hello world", &v);
    assert_eq!("Hello world world", &v2);
    assert_eq!("moto", &s);
}

// 7. 类型没有实现Copy，闭包中是&mut T操作，捕获的变量被消耗
// f 闭包对 s 变量，调用push_str(&mut self, &str)方法修改，s 被闭包消耗
// 此处捕获到的是 s 本身，s 被 move 到闭包中，闭包外部 s 就不可见了。
// f 是按照FnOnce trait 方式执行，不可以多次执行 f
fn test_fn_once_string() {
    let mut s = "Hello".to_owned();
    let f = || -> String {
        s.push_str(" world");
        s // s被消耗
    };

    // s被move进f闭包中，s被消耗，是FnOnce trait
    let v = f();

    // s变量已经被move了，不能再被borrowed
    // dbg!(&s);

    // f只能调用一次
    // let v2 = f();

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例
    s = "moto".to_owned();

    assert_eq!("Hello world", v);
    assert_eq!("moto", &s);
}

// 8. 类型没有实现Copy，闭包使用move关键字，闭包中是T操作，捕获的变量被消耗
// f 闭包对 s 变量，调用into_boxed_str(self)方法，s 被闭包消耗，
// 此处捕获到的是 s 本身，s 被 move 到闭包中，闭包外部 s 就不可见了
// f 是按照FnOnce trait 方式执行，不可以多次执行 f
// 本例中move关键字不是必须的
fn test_fn_once_move_string() {
    let mut s = "Hello".to_owned();
    let f = move || s.into_boxed_str();

    // s被move进f闭包中，s被消耗，是FnOnce trait
    let v = f();

    // s变量已经被move了，不能再被borrowed
    // dbg!(&s);

    // f只能调用一次
    // let v2 = f();

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例
    s = "moto".to_owned();

    assert_eq!("Hello", &*v);
    assert_eq!("moto", &s);
}

fn main() {
    // test_fn_i8();
    // test_fn_mut_i8();
    // test_fn_mut_i8_move();
    // test_fn_string();
    // test_fn_mut_string();
    // test_fn_mut_move_string();
    // test_fn_once_string();
    test_fn_once_move_string();
}
