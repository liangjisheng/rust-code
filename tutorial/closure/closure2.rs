// 实际上，一个闭包并不仅仅实现了某一种 Fn trait，其规则如下：
// 所有的闭包都自动实现了 FnOnce trait，因此任何一个闭包都至少可以被调用一次；
// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut trait；
// 不需要对捕获变量进行改变的闭包自动实现了 Fn trait

fn exec<F: FnOnce()>(f: F) {
    // 规则1: 所有闭包自动实现FnOnce
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    // 规则2: 没有移出捕获变量所有的闭包自动实现FnMut
    f()
}

fn exec2<F: Fn()>(f: F) {
    // 规则3: 不需要对捕获变量进行改变的自动实现Fn
    f()
}

fn c1() {
    let s = String::from("hello");
    // 闭包进行了不可变借用，没有移出变量所有权，不改变变量
    // 虽然闭包只是对 s 进行了不可变借用，实际上，它可以适用于任何一种 Fn trait
    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec3<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    // 错误，实现了FnOnce
    f("hello");
}

fn c2() {
    let mut s = String::new();
    // 闭包从捕获环境中移出了变量s的所有权，并进行改变，因此闭包只实现了 FnOnce，未实现 FnMut 和 Fn
    let update_string = |str| -> String {
        s.push_str(str);
        s
    }; // 移出了s所有权

    // exec3(update_string);
}

// 实现 Fn 的前提是实现 FnMut，FnMut 的前提是实现 FnOnce，因此要实现 Fn 就要同时实现
// FnMut 和 FnOnce。 在实际项目中，建议先使用 Fn ，然后编译器会告诉你正误以及该如何选择

// Rust要求函数的参数和返回类型，必须有固定的内存大小。绝大部分类型都有固定的大小，
// 但是不包括trait，因为编译器无法知道其真实类型或具体大小，因此编译器会提示使用 impl 关键字

// fn return_closure(x: i32) -> impl Fn(i32) -> i32 {
//     let num = 5;
//
//     if x > 1 {
//         move |x| x + num
//     } else {
//         move |x| x - num
//     }
// }

// 但是上面代码不能通过，因为两个分支返回了不同的闭包类型。即使是签名相同的闭包，
// 类型也是不同的。可以用 Box 实现特征对象

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    // dyn是申明trait对象类型的关键字
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn main() {
    // c1();
    c2();
}

// 闭包类型占用内存的大小
// 如果把闭包理解成一个结构体，闭包所捕获的变量相当于结构体中的字段，则闭包的大小就和闭包参数
// 闭包代码的局部变量都没有关系，而只与闭包从其环境中捕获的变量有关
