// 解构赋值时，可使用_作为某个变量的占位符，使用..作为剩余所有变量的占位符
// (使用..时不能产生歧义，例如(..,x,..)是有歧义的)。当解构的类型包含了命名字段时，
// 可使用 field_name 简化 field_name: field_name 的书写

struct Point2 {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum IPAddr {
    IPAddr4(u8, u8, u8, u8),
    IPAddr6(String),
}

fn p1() {
    // 解构Struct
    let p = Point2 { x: 0, y: 7 };

    // 等价于 let Point2{x: x, y: y} = p;
    let Point2 { x, y } = p;
    println!("x: {}, y: {}", x, y);
    // 解构时可修改字段变量名: let Point2{x: a, y: b} = p;
    // 此时，变量a和b将被赋值

    let ori = Point3 { x: 1, y: 0, z: 0 };
    match ori {
        // 使用..忽略解构后剩余的字段
        Point3 { x, .. } => println!("{}", x),
    }

    // 解构enum
    let ipv4 = IPAddr::IPAddr4(127, 0, 0, 1);
    match ipv4 {
        // 丢弃解构后的第四个值
        IPAddr::IPAddr4(a, b, c, _) => println!("{},{},{}", a, b, c),
        IPAddr::IPAddr6(s) => println!("{}", s),
    }

    // 解构元组
    let ((feet, inches), Point2 { x, y }) = ((3, 1), Point2 { x: 3, y: -1 });
    println!("feet: {}", feet);
    println!("inches: {}", inches);
    println!("x: {}, y: {}", x, y);
}

struct S(i32, i32);

fn p2() {
    // 当解构后进行模式匹配时，如果某个值没有对应的变量名，则可以使用@手动绑定一个变量名
    match S(1, 2) {
        // 如果匹配1成功，将其赋值给变量z
        // 如果匹配2成功，也将其赋值给变量z
        S(z @ 1, _) | S(_, z @ 2) => {
            println!("z {}", z);
            assert_eq!(z, 1)
        }
        _ => panic!(),
    }

    // 匹配并解构一个数组
    let arr = ["x", "y", "z"];
    match arr {
        [.., "!"] => println!("!!!"),
        // 匹配成功，start = ["x", "y"]
        [start @ .., "z"] => println!("starts: {:?}", start),
        ["a", end @ ..] => println!("ends: {:?}", end),
        rest => println!("{:?}", rest),
    }
}

fn main() {
    // p1();
    p2();
}
