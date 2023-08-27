// 当我们定义的 struct 的里面有对象引用的时候，我们需要在 struct 的模板参数中增加生命周期声明
// 在这里生命周期'a便是part的生命周期，编译器会在编译的时候检查ImportantExcerpt和'a
// 如果 ImportantExcerpt 的生命周期大于 'a，则会报错

// 这个结构体有唯一一个字段 part，它存放了一个字符串 slice，这是一个引用
// 类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数，
// 以便在结构体定义中使用生命周期参数。这个注解意味着 ImportantExcerpt
// 的实例不能比其 part 字段中的引用存在的更久

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予
// &self 和 announcement 他们各自的生命周期。接着，因为其中一个参数是 &self
// 返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn s1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 跟函数生命周期声明类似，当一个生命周期参数修饰多个字段的时候
// 编译器会将这个生命周期参数推断出这几个字段生命周期最小的那个

struct Foo<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn s2() {
    // let x = 6;
    // let m;
    //
    // {
    //     let y = 6;
    //     let f = Foo { x: &x, y: &y };
    //     m = f.x;
    // }
    //
    // println!("{}", m);
}

// 在上述例子中，Foo包含一个生命周期参数，这个生命周期参数修饰了x和y结构体字段
// 'a将会被编译器推断为x和y的生命周期中最小的那个，在这个例子中，
// 'a会被推断为y的生命周期。 在这个例子中，编译器会报错，因为x的生命周期被声明为和y的生命周期一样
// 所以当打印m的时候，x会被编译器认为已经无效

// 将我们的代码修改成如下: 此时'a将会被推断为x的生命周期，'b将会被推断为y的生命周期，编译将会通过
struct Foo1<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn s3() {
    let x = 6;
    let m;

    {
        let y = 6;
        let f = Foo1 { x: &x, y: &y };
        m = f.x;
    }

    println!("{}", m);
}

struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

// impl<'a> Parser<'a> {
//     fn parse(&self) -> Result<(), &str> {
//         Err(&self.context.0[1..])
//     }
// }

// fn parse_context<'a>(context: &'a Context<'a>) -> Result<(), &'a str> {
//     Parser { context }.parse()
// }

// 上面的代码编译时编译器说返回的对象是局部的！问题在哪里？ 原来是Parser的parse
// 方法的默认返回值的生命周期和Parser对象是一致的，所以parse返回的引用在
// parse_context结束的时候就无效了，因此编译器报错了，埋怨引用了局部对象

// 找到了问题的症结，我们需要显示生命parse方法的返回值为'a生命周期
// 而不是默认的Parser的生命周期，也就是self的生命周期，这里指的就是 parse_context 的函数内部
impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &'a str> {
        Err(&self.context.0[1..])
    }
}

// 编译通过，我们可以引用Parser里面的内部变量了
fn parse_context<'a>(context: &'a Context<'a>) -> Result<(), &'a str> {
    Parser { context }.parse()
}

// 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）
// 这并不是需要程序员遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑
// 如果代码符合这些场景，就无需明确指定生命周期
// 编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期
// 后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用
// 编译器将会停止并生成错误

// 第一条规则是编译器为每一个引用参数都分配一个生命周期参数
// 函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)
// 有两个引用参数的函数就有两个不同的生命周期参数
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
// fn foo<'a>(x: &'a i32) -> &'a i32

// 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self
// 说明是个对象的方法, 那么所有输出生命周期参数被赋予 self 的生命周期。
// 第三条规则使得方法更容易读写，因为只需更少的符号

fn main() {
    // s1();
    // s2();
    // s3();
}
