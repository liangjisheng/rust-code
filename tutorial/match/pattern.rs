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

fn p2() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn p3() {
    // match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

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
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn p5() {
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

// 下划线（_）是可以匹配但不绑定任何值的通配符模式，可以将其用于任意模式，包括函数参数中
// 这段代码会完全忽略作为第一个参数传递的值 3
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn p6() {
    foo(3, 4);

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
    //报错
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
}

// 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，
// 它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况
fn p9() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

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
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message1::Hello { id } => println!("id {}", id),
    }
}

fn main() {
    // p1();
    // p2();
    // p3();
    // p4();
    // p5();
    // p6();
    // p7();
    // p8();
    // p9();
    p10();
}
