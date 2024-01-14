use macro_demo::*;

// macro converts struct S to struct H
#[my_custom_attribute]
struct S {}

// 编译期间会打印结构类型和参数
#[log_attr(struct, "world")]
struct Hello {
    pub name: String,
}

#[log_attr(func, "test")]
fn invoke() {}

#[log_attr]
fn invoke1() {}

#[log_attr(bar)]
fn invoke2() {}

#[log_attr(multiple => tokens)]
fn invoke3() {}

#[log_attr { delimiters }]
fn invoke4() {}

#[test]
fn test_attribute_macro() {
    // due to macro we have struct H in scope
    let demo = H {};

    let v1 = Hello {
        name: String::from("ljs"),
    };

    invoke();
    invoke1();
    invoke2();
    invoke3();
    invoke4();
}

#[test]
fn test_function_like_macro() {
    /// 使用我们定义的宏，生成一个函数
    say_hello!();
    /// 调用函数
    println!("{}", hello())
}

#[test]
fn test_function_macro() {
    a_proc_macro!();
    println!("{}", anwser());

    make_hello!(world);
    make_hello!(张三);

    hello_world();
    hello_张三();
}

#[derive(Hello)]
struct World {
    pub name: String,
}

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

pub trait Trait {
    fn print(&self) -> usize;
}

#[derive(Trait)]
struct Swap;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: String,
}

#[test]
fn test_derive_macro() {
    let v1 = World {
        name: String::from("world"),
    };
    // println!("{}", &v1.name);

    // Pancakes::hello_macro();

    let s = Swap {};
    let n1 = s.print();
    println!("n1: {}", n1);
    // Swap::print(&s);

    // let mut builder = Command::builder();
    // builder.executable("cargo".to_owned());
    // builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    // builder.env(vec![]);
    // builder.current_dir("..".to_owned());

    // assert_eq!(builder.executable, "cargo");
    // assert_eq!(builder.args, &["build", "--release"]);
    // assert_eq!(builder.current_dir, "..");
}
