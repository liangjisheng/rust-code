// 完全限定语法与消歧义：调用相同名称的方法
// 当一个类型T同时实现了多个trait T1、T2、T3，这三个trait都有相同名字的方法func，
// 同时T自身也有一个func，即T实现了4个不同的func。如果有一个变量t: T，默认情况下
// 调用的是T类型的func，而不是另外三种trait的func。如果要调用T1的func，
// 则需要以T1::func(&t)或T1::func(t)的形式调用。

// 如果T、T1、T2、T3同名的是关联函数(即静态方法)a_func，则t: T默认调用的也是
// a_func，要调用T1的a_func，就需要使用完全限定语法消除歧义

// <T as T1>::a_func()
// <Type as Trait>::function(receiver_if_method, next_arg, ...);

struct Human;

trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

trait Wizard {
    fn fly(&self);
    fn name() -> String;
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

    fn name() -> String {
        "Pilot".to_string()
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }

    fn name() -> String {
        "Wizard".to_string()
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }

    fn name() -> String {
        "Human".to_string()
    }
}

fn main() {
    let person = Human;

    // This is your captain speaking.
    Pilot::fly(&person);

    // Up!
    Wizard::fly(&person);

    // *waving arms furiously*
    person.fly();

    // Pilot
    // Pilot::name(); // 编译错误
    // 因为 Pilot::name 没有 self 参数，同时这可能会有其它类型实现了 Pilot trait，
    // Rust 无法计算出所需的是哪一个 Pilot::name 实现。我们会得到这个编译错误：

    // 使用完全限定语法消除歧义
    <Human as Pilot>::name();

    // Wizard
    <Human as Wizard>::name();

    // Human
    Human::name();
}
