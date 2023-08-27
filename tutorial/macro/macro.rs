// 声明式宏
// Rust宏让你可以发明自己的语法，编写出可以自行展开的代码，也就是我们通常所说的元编程，
// 你甚至可以用Rust宏来创作自己的DSL
// Rust宏的基本运作机制就是：首先匹配宏规则中定义的模式，然后将匹配结果绑定到变量，最后展开变量替换后的代码
// Rust 的宏与 C 语言中的宏有很大不同，Rust 的宏是应用于词法树，而 C 语言的宏只是文本替换

// Rust 有 2 种类型的宏
// 声明式宏 （Declarative macros）使你能够编写类似于 match 表达式的东西，
// 对你提供的作为参数的 Rust 代码进行操作。它使用你提供的代码来生成取代宏调用的代码
// 过程宏 （Procedural macros）允许你对它所给的 Rust 代码的抽象语法树（AST）进行操作
// 一个 proc 宏是一个从一个TokenStream'（或两个）到另一个TokenStream'的函数，
// 其中的输出取代了宏的调用

// 左边的小括号部分是Rust宏的匹配器/Matcher，用来匹配模式并捕捉变量，这是我们发明自定义语法和DSL的关键所在
// 右边的大括号部分是Rust宏的转码器/Transcriber，也就是我们要应用匹配器捕捉到的变量的部分，
// Rust编译器将利用变量和这部分的代码来生成实际的Rust代码

// 表达式选择器只是Rust中众多可用选择器中的一个，下面是一些常见的Rust宏选择器
// item: 条目，例如函数、结构、模块等
// block: 代码块
// stmt: 语句
// pat: 模式
// expr: 表达式
// ty: 类型
// ident: 标识符
// path: 路径，例如 foo、 ::std::mem::replace, transmute::<_, int>, …
// meta: 元信息条目，例如 #[…]和 #![rust macro…] 属性
// tt: 词条树
// vis: 一个可能为空的Visibility限定词

// 规则所用的括号可以是以下的任何一种：()，[]，{}。并且在调用的时候并不会对所用的括号进行检查
// 也就是说，你可以用 foo! {} 来调用定义为 macro_rules! foo { () => {} } 的规则

// 可以定义多条宏规则
macro_rules! hey {
    // () => {};
    // 在匹配器/Matcher中，$name部分定义了变量名，匹配结果将绑定到该变量以便应用到转码器/Transcriber中
    // 冒号后面的部分被称为选择器/Designator，用于声明我们要匹配的类型。例如在这个示例中，
    // 我们使用的是表达式选择器，也就是expr，这告诉Rust：匹配一个表达式，然后存入$name变量
    ($name:expr) => {
        println!("Hey {}", $name);
    };
}

macro_rules! multi_hey {
    // 重复模式的匹配
    // 只需要把希望重复的模式写在$(...)这部分，然后插入分隔符
    // 在这里也就是逗号，最后添加一个*符号，表示重复匹配$()中的模式
    ( $( $name:expr ),*) => {{
        $( println!("multi_hey {}", $name); )*
    }};
}

use std::collections::HashMap;

macro_rules! map {
    ($( $key:expr => $value:expr ) ,*) => {{
        let mut hm = HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    }};
}

macro_rules! swap_value {
    ($a:expr,$b:expr) => {
        let temp;
        temp = $b;
        $b = $a;
        $a = temp;
    };
}

macro_rules! lut {
    ("TRIGER") => {
        0x100
    };
    ("WRITE") => {
        0x200
    };
}

macro_rules! lut_v1 {
    (TRIGER) => {
        0x300
    };
    (WRITE) => {
        0x400 + lut_v1!(TRIGER)
    };
}

#[test]
fn test_macro() {
    let mut test_val_1 = 0x100;
    let mut test_val_2 = 0x200;
    swap_value!(test_val_1, test_val_2);
    assert_eq!(test_val_1, 0x200);
    assert_eq!(test_val_2, 0x100);
}

#[test]
fn test_marco_reg() {
    assert_eq!(lut!("TRIGER"), 0x100);
    assert_eq!(lut!("WRITE"), 0x200);
}

#[test]
fn test_marco_reg_v1() {
    assert_eq!(lut_v1!(TRIGER), 0x300);
    assert_eq!(lut_v1!(WRITE), 0x400 + lut_v1!(TRIGER));
}

// 这是一个简单的宏，名为 `say_hello`
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数
    () => {
        // 此宏将会展开成这个代码块里面的内容
        println!("Hello!");
    };
}

// 宏的每个分支接收一个函数的参数，并且参数可以被指定多个类型。
// 如果想要add函数也能仅接收一个参数，我们可以添加另一个分支
macro_rules! add {
    // match add!(1), add!(2) etc
    ($a:expr) => {{
        $a
    }};

    // match like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };

    // 第三个分支使用剩下的参数再次调用add宏
    ($a:expr,$($b:tt)*)=>{
       {
           $a+add!($($b)*)
       }
    }
}

macro_rules! add_as {
    // using a ty token type for matching datatype passed to macro
    ($a:expr,$b:expr,$typ:ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! multi_add{
    (
  // repeated block
  $($a:expr)
 // seperator
   ,
// zero or more
   *
   )=>{
        {
   // to handle the case without any arguments
   0
   // block to be repeated
   $(+$a)*
     }
    }
}

fn main() {
    hey!("hello");

    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v1: {:?}", v1);

    multi_hey!("alice", "bob");

    let user = map!(
        "name" => "alice",
        "gender" => "girl"
    );
    println!("user: {:?}", user);

    say_hello!();

    // 匹配时，宏会被匹配分支的代码替换
    // 这个宏没有对两个数执行相加操作，它只是把自己替换为把两个数相加的代码
    // println!("add: {}", 1+2);
    println!("add: {}", add!(1, 2));
    // println!("a1: {}", 1);
    println!("a1: {}", add!(1));
    println!("a1: {}", add!(1, 2, 3, 4));

    println!("{}", add_as!(0, 2, u8));
    println!("{}", multi_add!(0, 1, 2, 3));
}
