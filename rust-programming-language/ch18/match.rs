// Rust 提供的匹配模式允许将一个值与一系列的模式比较，并根据匹配的模式执行相应的代码块，使用表达式 match 表示
// match 语句有返回值，它把 匹配值 后执行的最后一条语句的结果当作返回值

// get_url_by_language 根据语言获取一个对应的地址，match 表达式的结果就是这个函数的结果
// 看起来有点像 if 表达式，但是 if 只能返回 true 或 false，match 表达式可以返回任何类型

#[derive(Debug)]
enum OpenJS {
    Nodejs,
    React,
}
enum Language {
    Go,
    Rust,
    JavaScript(OpenJS),
}

// 如果 Go 匹配，因为这个分支我们仅需要返回一个值，可以不使用大括号
// 如果 Rust 匹配，这次我们需要在分支中执行多行代码，可以使用大括号
// 如果 JavaScript 匹配，这次我们想对匹配的模式绑定一个值，可以修改枚举的一个成员来存放数据，这种模式称为绑定值的模式

fn get_url_by_language(language: Language) -> String {
    match language {
        Language::Go => String::from("https://golang.org/"),
        Language::Rust => {
            println!("We are learning Rust.");
            String::from("https://www.rust-lang.org/")
        }
        Language::JavaScript(value) => {
            println!("Openjs value {:?}!", value);
            String::from("https://openjsf.org/")
        }
    }
}

// Option 是 Rust 系统定义的一个枚举类型，它有两个变量：None 表示失败
// Some(value) 是元组结构体，封装了一个范型类型的值 value

// Rust 匹配模式还有一个概念**匹配是穷尽的，**上例中 None => None 是必须写的，
// 否则会报 pattern None not covered 错误，编译阶段就不会通过的

fn something(num: Option<i32>) -> Option<i32> {
    match num {
        // Option::None => Option::None,
        // Option 是系统定义的, 所以可以省略
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn match1() {
    print!(
        "{}\n",
        get_url_by_language(Language::JavaScript(OpenJS::Nodejs))
    );
    print!(
        "{}\n",
        get_url_by_language(Language::JavaScript(OpenJS::React))
    );
    print!("{}\n", get_url_by_language(Language::Go));
    print!("{}\n", get_url_by_language(Language::Rust));
}

fn match2() {
    let five = Some(5);
    let six = something(five);
    let none = something(None);
    println!("{:?} {:?}", six, none);

    // 我们想仅在 Some(value) 匹配时做些处理，其它情况不想考虑，为了满足 match 表达式穷进性的要求
    // 还要在写上 _ => () 来匹配其它的情况
    let five = Some(5);
    match five {
        Some(value) => println!("{}", value),
        _ => (),
    }

    // 只针对一种模式做匹配处理的场景下，可以使用 if let 语法，可以更少的代码来写
    if let Some(value) = five {
        println!("{}", value + 1);
    }

    // 可以使用简单的条件作为判断
    let num = 3;
    match num {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }
}

fn match3() {
    // 如果写一个固定的值，即单个值匹配
    // 使用 | 符号实现多值匹配
    // 使用 ..= 符号实现范围匹配，注意，之前是使用 ... 现在该方式已废弃
    // _ 符号是匹配穷进行，Rust 要检查所有被覆盖的情况

    // 编译器会在编译时检查范围不为空，而 char 和数字值是 Rust 仅有的可以判断范围是否为空的类型，
    // 所以范围只允许用于数字或 char 值。

    let week_day = 0;
    match week_day {
        1..=4 => println!("周一至周四过的好慢啊..."),
        5 => println!("哇！今天周五啦！"),
        6 | 0 => println!("这两天是周末，休息啦！"),
        _ => println!("每周只有 7 天，请输入正确的值..."),
    };

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    };
}

fn match4() {
    let str1 = "Imagine";
    let str2 = "Miracle";

    match str1 {
        str if (str == "Imagine") => println!("Imagine"),
        str if (str == "Miracle") => println!("Miracle"),
        _ => println!("Others"),
    }

    match str2 {
        str if (str == "Imagine") => println!("Imagine"),
        str if (str == "Miracle") => println!("Miracle"),
        _ => println!("Others"),
    }

    let state_code = "MS";
    let state = match state_code {
        "MH" => {
            println!("Found match for MH");
            "Maharashtra"
        }
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown",
    };
    println!("State name is {}", state);
}

fn main() {
    // match1();
    // match2();
    // match3();
    match4();
}
