use std::panic;

fn unwrap_option(option: Option<i32>) -> i32 {
    option.unwrap()
}

// 使用 panic::catch_unwind 来接住 panic，并打印结果，如果有的话
fn unwrap_print_option(f: fn() -> i32) {
    if let Ok(val) = panic::catch_unwind(f) {
        println!("val = {}", val);
    }
}

fn op1() {
    println!(">>>>> test Option.unwrap");

    unwrap_print_option(|| unwrap_option(None));
    unwrap_print_option(|| unwrap_option(Some(6)));
}

fn create_option(x: i32) -> Option<i32> {
    if x >= 0 {
        Some(x)
    } else {
        None
    }
}

fn check_positive_integer(x: i32) {
    if let None = (|| {
        let val = create_option(x)?;

        println!("get positive integer: {}", val);

        Some(0)
    })() {
        println!("{} is not positive integer", x);
    }
}

// 使用 ? 语法解构
fn test_question_syntax() {
    println!(">>>>> test Option?");

    check_positive_integer(100);
    check_positive_integer(0);
    check_positive_integer(-100);
}

fn main() {
    // op1();
    test_question_syntax();
}
