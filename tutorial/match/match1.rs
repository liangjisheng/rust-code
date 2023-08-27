enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn m1() {
    let v = Coin::Penny;
    let v1 = value_in_cents(v);
    println!("{}", v1);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 返回unit值，统一各个分支的返回情况
    }
}

fn m2() {
    // 解构含义为从匹配模式的给定值中提取出对应的字段值
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a {}, b {}", a, b); // a = 0; b = 7;

    // 使用 _ 忽略部分值或者整个值
    let Point { x: _, y: c } = p;
    println!("c {}", c); // c = 7;

    let Point { x: d, y: _ } = p; // d = 0;
    println!("d {}", d);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet {}, inches {}", feet, inches);
    println!("x {}, y {}", x, y);

    struct Example {
        a: bool,
        b: u64,
    }

    let Example { a, b: _ } = Example { a: true, b: 10004 };
    assert!(a);
}

// 解构枚举
fn m3() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// 解构嵌套的结构体和枚举
fn m4() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

fn m5() {
    // 通过在名字前以一个下划线开头来忽略未使用的变量
    // 使用_和_变量名两种语法是不一样的：前者值没有被绑定，后者还是会发生值的绑定
    let _x = 5;
    let y = 10;
    let _ = 10;

    // 用 .. 忽略剩余值
    // .. 忽略剩余值的语法可以看成是_忽略值的一个语法糖，通常使用_忽略模式中的值往往
    // 是需要忽略的值比较少的使用才会用到，当需要忽略的值非常多，想要提取的值比较少时
    // 使用..忽略会更方便
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 1, y: 2, z: 3 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 只提取元组中首尾的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// 匹配守卫提供条件判断, 匹配守卫（match guard） 是一个指定于 match
// 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
// 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况
fn m6() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

// at 运算符（@）允许开发者在创建一个存放值的变量的同时测试其值是否匹配模式
fn m7() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // 匹配结构体，并将id字段的值提取出来到id_variable中，然后判断是否属于[3,7]
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn main() {
    // m1();
    // m2();
    // m3();
    // m4();
    // m5();
    // m6();
    m7();
}
