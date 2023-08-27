// isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台
// 如果是 32 位架构的处理器将使用 32 位位长度整型

// 为了方便阅读超大的数字，Rust 语言允许使用一个虚拟的分隔符 也就是下划线（ _ ） 来对数字进行可读性分隔符
// 比如为了提高 50000 的可读性，我们可以写成 50_000
// Rust 语言会在编译时移除数字可读性分隔符 _

fn data_type() {
    let num10 = 10_000; // 等于 10000
    let num16 = 0xff; // 十六进制
    let num8 = 0o12; // 8进制 等于十进制的 10
    let num2 = 0b1111_0000; // 二进制
    let numBytes = b'A'; // 字节(只能表示 u8 型)

    println!("{}", num10); // 10000
    println!("{}", num16); // 255
    println!("{}", num8); // 10
    println!("{}", num2); // 240
    println!("{}", numBytes); // 65

    let numArray: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numArray); // [1, 2, 3, 4, 5]

    let ni8 = 1i8;
    let nu8 = 1u8;
    let ni16 = 1i16;
    let nu16 = 1u16;
    let ni32 = 1i32;
    let nu32 = 1u32;
    let ni64 = 1i64;
    let nu64 = 1u64;
    let ni128 = 1i128;
    let nu128 = 1u128;
    let nisize = 1isize;
    let nusize = 1usize;

    let c1 = 'a'; // 字符类型的字面值使用单引号 占用4字节大小
    let special_character = '@';
    let alphabet: char = 'A';
    println!("special_character is {}", special_character);
    println!("alphabet is {}", alphabet);

    let tup: (i32, f64, u8) = (500, 7.3, 2);
    println!("{}, {}, {},", tup.0, tup.1, tup.2);
}

fn data_type2() {
    // 默认是 f64
    let result = 10.00;
    // 单精度浮点型
    let interest: f32 = 8.35;
    // 双精度浮点型
    let cost: f64 = 15000.600;

    println!("result is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);

    const USER_LIMIT: i32 = 100;
    const PI: f32 = 3.14;
    println!("user limit is {}", USER_LIMIT);
    println!("pi value is {}", PI);
}

fn data_type3() {
    let num = 1;
    // num = 2; // 编译失败 cannot assign twice to immutable variable 'num'
    println!("num {}", num);

    // 在变量名称前加上 mut 关键字，表明该变量是可变的

    let mut num = 1;
    println!("num {}", num);
    num = 2;
    println!("num {}", num);

    // 常量使用 const 声明，之后是不可变的，在声明时必须指定变量类型，这也是与 let 的不同
    const NUM: i8 = 1;
    println!("const {}", NUM);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {}, y: {}", x, y); // x: 2, y: 3

    let x = true; // bool
    let y: bool = false; // bool
    println!("x: {}, y: {}", x, y); // x: 2, y: 3

    // Rust 中的字符型为一个 Unicode 码，大小为 4 bytes，char 类型使用单引号包括
    let x = 'x';
    let y = ' ';
    println!("x: {}, y: {}", x, y); // x: x, y:

    // const 修饰的常量是不允许使用 mut 的，即常量必须保证自己是不可变的，这一点是不可违反的
    // 使用 const 修饰声明的常量必须注明其数据类型，而 let 则可以不显示注明，依靠 Rust 编译器自动识别变量类型
    // 常量只能赋值为常量表达式，而非仅再运行时计算的数值
    // Rust 中的常量命名规则，需使用全大写的单词，之间需要用下划线间隔
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn main() {
    data_type();
    data_type2();
    data_type3();
}
