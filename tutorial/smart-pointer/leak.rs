// 要知道真正具有 'static 生命周期的往往都是编译期就创建的值
// 比如字符串字面量, const 常量

// 关联函数 Box::leak 它可以消费掉Box并且强制目标值从内存中泄漏
// Box::leak可以用于全局变量，例如用作运行期初始化的全局动态配置

// Box::leak 可以将一个局部生命周期的变量变为全局生命周期的变量
// 可以把该变量赋值给一个全局变量，实现在运行期初始化全局变量的目的
// lazy_static 宏也可以实现这一点，但他并没有 Box::leak 灵活
// 后者可以应用在很多场景中

fn get_static_str() -> &'static str {
    static mut s: String = String::new();
    unsafe {
        s.push_str("hello");
        &s
    }
}

fn get_static_str1() -> &'static str {
    let mut s = String::new();
    s.push_str("hello");

    // 使用Box::leak就可以将一个运行期的值转为 'static
    Box::leak(s.into_boxed_str())
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut config: Option<&mut Config> = None;

fn l1() {
    // 试图将一个局部生命周期的变量赋值给全局生命周期的config，这明显是不安全的
    // unsafe {
    //     config = Some(&mut Config {
    //         a: "a".to_string(),
    //         b: "b".to_string(),
    //     });
    //
    //     println!("{:?}", config);
    // }

    let c = Box::new(Config {
        a: "a".to_string(),
        b: "b".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        config = Some(Box::leak(c));
        println!("{:?}", config);
    }
}

// 从函数中返回全局变量
fn l2() -> Option<&'static mut Config> {
    // 直接返回临时值是不行的
    // returns a value referencing data owned by the current function
    // Some(&mut Config {
    //     a: "a".to_string(),
    //     b: "b".to_string(),
    // })

    let c = Box::new(Config {
        a: "a".to_string(),
        b: "b".to_string(),
    });

    Some(Box::leak(c))
}

fn main() {
    // println!("{}", get_static_str());
    // println!("{}", get_static_str1());

    // l1();

    unsafe {
        config = l2();
        println!("{:?}", config);
    }
}
