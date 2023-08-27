// 生命周期的3条规则

// 1. 每一个是引用的参数都有它自己的生命周期参数, 生命周期属于引用对象本身
fn f(x: &i32) {}
fn f1<'a>(x: &'a i32) {}

// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
fn f2(x: &String) -> &String {
    x
} // 返回生命周期同参数 x

//显示标注，返回生命周期同参数 x
fn f3<'a>(x: &'a String) -> &'a String {
    x
}

//为什么输出值一定是 'a 即使没有显示标注？
//因为输出值是引用，如果引用不属于输入参数，那么只能是内部参数的引用，此时就会产生悬垂引用。
//这是不被允许的

// 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self
// 那么在非标注生命周期时，所有输出生命周期参数被赋予 self 的生命周期
// 标注生命周期之后则取生命周期较短者，即非 self

struct ABC<'a> {
    field1: &'a String,
}

impl<'a> ABC<'a> {
    //此时没有显示标注生命周期，所以默认返回值为self的生命周期
    fn f1(&self, a: &String) -> &String {
        // a //错误，因为生命周期不同
        &self.field1 //ok
    }

    //标注生命周期之后，根据单一生命周期标注规则，输出值生命周期取生命周期较短者
    //所以此时返回a是OK的
    //这里返回值标注可以取消，默认即 &'a
    fn f2(&self, a: &'a String) -> &'a String {
        // 但是可以这样
        a // ok

        // &self.field1 //错误，因为生命周期不同
    }
}

use std::fmt::Display;

// 结合泛型类型参数、trait bounds 和生命周期
// 返回两个字符串 slice 中较长者的 longest 函数，不过带有一个额外的参数 ann
// ann 的类型是泛型 T，它可以被放入任何实现了 where 从句中指定的 Display trait
// 的类型。这个额外的参数会使用 {} 打印，这也就是为什么 Display trait bound 是必须的
// 因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn f4() {
    let x = "alice";
    let y = "bob";
    let i = 0;
    let s = longest_with_an_announcement(x, y, i);
    println!("s {}", s);
}

fn main() {
    f4();
}
