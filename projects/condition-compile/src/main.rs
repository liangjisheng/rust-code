// https://learnku.com/articles/41295
// https://blog.csdn.net/lcloveyou/article/details/106269617

// 条件编译可以通过两种不同的操作符实现，如下
// cfg 属性：在属性位置中使用 #[cfg (…)]
// cfg！宏：在布尔表达式中使用 cfg!(…)

// 仅当目标系统为Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("linux!")
}

// 仅当目标系统不是Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_not_on_linux() {
    println!("not linux!")
}

// 直接使用 rustc 传递条件进行编译, 如果想要编译器编译的话
// rustc --cfg some_condition main.rs
// cargo build --features some_condition
// cargo run --features some_condition

#[cfg(feature = "some_condition")]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    // 如果目标系统不是 linux， 则不能调用，因为都没有编译
    // are_you_on_linux();

    are_you_not_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's linux!");
    } else {
        println!("Yes. It's not linux!");
    }

    conditional_function();
}
