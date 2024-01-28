fn p1() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// if let 也可以像 match 分支那样引入覆盖变量：if let Ok(age) = age 引入了一个新的覆盖变量 age，它包含 Ok 成员中的值。

fn p2() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 只要模式匹配就一直进行 while 循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn p3() {
    // 每一次像这样使用 let 语句就是在使用模式！let 语句更为正式的样子如下
    // let PATTERN = EXPRESSION;
    // 像 let x = 5; 这样的语句中变量名位于 PATTERN 位置，变量名不过是形式特别朴素的模式。
    // 我们将表达式与模式比较，并为任何找到的名称赋值。所以例如 let x = 5; 的情况，x 是一个
    // 代表 “将匹配到的值绑定到变量 x” 的模式。同时因为名称 x 是整个模式，这个模式实际上等于
    // “将任何值绑定到变量 x，不管值是什么”。
    let x = 5;

    // 这里将一个元组与模式匹配。Rust 会比较值 (1, 2, 3) 与模式 (x, y, z) 并发现此值匹配这个模式
    let (x, y, z) = (1, 2, 3);

    // match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量

    // 第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为我们在 match 表达式
    // 的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 y。这个新的 y 绑定会匹配任何 Some
    // 中的值，在这里是 x 中的值。因此这个 y 绑定了 x 中 Some 内部的值。这个值是 5，所以这个分支的
    // 表达式将会执行并打印出 Matched, y = 5。

    // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。最后的 println!
    // 会打印 at the end: x = Some(5), y = 10
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的
// 模式被称为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以
// 匹配任何值所以不可能会失败。对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。
// 一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的
// 值是 None 而不是 Some，那么 Some(x) 模式不能匹配。
// match匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。

// 也可以使用模式来解构结构体、枚举和元组，以便使用这些值的不同部分。
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn p4() {
    let p = Point { x: 0, y: 7 };
    // 创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 但是通常是变量名匹配字段名，所以对于匹配结构体字段的模式存在简写
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 也可以使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 对于像 Message::Quit 这样没有任何数据的枚举成员，不能进一步解构其值。
// 只能匹配其字面值 Message::Quit，因此模式中没有任何变量。
// 对于像 Message::Move 这样的类结构体枚举成员，
// 可以采用类似于匹配结构体的模式；对于像 Message::Write 这样的包含一个元素，
// 以及像 Message::ChangeColor 这样包含三个元素的类元组枚举成员，其模式则类似于用于解构元组的模式

// 解构嵌套的结构体和枚举
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor1(Color),
}

fn p5() {
    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Quit;
    // let msg = Message::Move { x: 1, y: 2 };
    let msg = Message::ChangeColor1(Color::Hsv(0, 160, 255));

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
        Message::ChangeColor1(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor1(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

// 对于数组，我们可以用类似元组的方式解构，分为两种情况
fn p5_1() {
    // 定长数组
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;

    assert_eq!(x, 114);
    assert_eq!(y, 514);

    // 不定长数组
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));
}

// 可以用复杂的方式来混合、匹配和嵌套解构模式
// let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

// 下划线（_）是可以匹配但不绑定任何值的通配符模式，可以将其用于任意模式，包括函数参数中
// 这段代码会完全忽略作为第一个参数传递的值 3
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn p6() {
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // 重要的部分是需要测试 setting_value 和 new_setting_value 都为 Some 成员的情况。
    // 在这种情况，我们打印出为何不改变 setting_value，并且不会改变它。
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // 这里忽略了一个五元元组中的第二和第四个值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// 只使用 _ 和使用以下划线开头的名称有不同：比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定
fn p7() {
    //报错 因为 s 的值会移动进 _s，并阻止我们再次使用 s。然而只使用下划线本身，并不会绑定值
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);

    //可以运行
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

// 对于有多个部分的值，可以使用 .. 语法来只使用部分并忽略其它值
struct Point1 {
    x: i32,
    y: i32,
    z: i32,
}

fn p8() {
    let origin = Point1 { x: 0, y: 0, z: 0 };
    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // 然而使用 .. 必须是无歧义的。如果期望匹配和忽略的值是不明确的，Rust 会报错。
    // Rust 不可能决定在元组中匹配 second 值之前应该忽略多少个值，以及在之后忽略多少个值。
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }
}

// 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，
// 它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况
fn p9() {
    // 无法在模式中表达类似 if x % 2 == 0 的条件，所以通过匹配守卫提供了表达类似逻辑的能力。
    // 这种替代表达方式的缺点是，编译器不会尝试为包含匹配守卫的模式检查穷尽性。
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // 也可以在匹配守卫中使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式。
    // 这个匹配条件表明此分支值匹配 x 值为 4、5 或 6 同时 y 为 true 的情况。
    let x = 4;
    let y = false;
    match x {
        //(4 | 5 | 6) if y => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
enum Message1 {
    Hello { id: i32 },
}

fn p10() {
    let msg = Message1::Hello { id: 5 };
    match msg {
        // 可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        // 第二个分支只在模式中指定了一个范围，分支相关代码没有一个包含 id 字段实际值的变量。
        // id 字段的值可以是 10、11 或 12，不过这个模式的代码并不知情也不能使用 id 字段中的值，
        // 因为没有将 id 值保存进一个变量。
        Message1::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        // 最后一个分支指定了一个没有范围的变量，此时确实拥有可以用于分支代码的变量 id，因为这里使用了
        // 结构体字段简写语法。不过此分支中没有像头两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。
        Message1::Hello { id } => println!("id {}", id),
    }
}

// 使用 @ 还可以在绑定新变量的同时，对目标进行解构
fn p11() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}

fn main() {
    // p1();
    // p2();
    // p3();
    // p4();
    // p5();
    // p5_1();
    // p6();
    // p7();
    // p8();
    // p9();
    // p10();
    p11();
}
