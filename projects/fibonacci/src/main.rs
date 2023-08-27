use std::io;

fn fibonacci_demo() {
    println!("The Fibonacci sequence demo: 0、1、1、2、3、5、8、13、21、34、...");
}

fn get_input_number() -> usize {
    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: usize = match number.trim().parse::<usize>() {
            Ok(num) => num,

            Err(_) => {
                println!("Please enter the correct index(like 1,2,3,...)!");

                continue;
            }
        };

        return number;
    }
}

fn overflow_check(num1: usize, num2: usize) -> bool {
    if num1 > (usize::MAX / 2) && num2 > (usize::MAX / 2) {
        return false;
    } else if num1 > (usize::MAX / 2) || num2 > (usize::MAX / 2) {
        let checker: isize = num1 as isize - (usize::MAX / 2) as isize;

        let checker1: usize = num2 - (usize::MAX / 2);

        let checker: isize = checker + checker1 as isize;

        if checker > 0 {
            return false;
        } else {
            return true;
        }
    } else {
        return true;
    }
}

fn fibonacci(index: usize) -> usize {
    let mut f0: usize = 0;
    let mut f1: usize = 1;
    let mut result: usize = f0;

    for _i in 1..index {
        if overflow_check(f0, f1) {
            result = f0 + f1;
            f0 = f1;
            f1 = result;
        } else {
            println!("It's too big!!!");
            return 0;
        }
    }

    match index {
        0 => 0,
        1 => 1,
        _ => result,
    }
}

// 递归实现
// 使用递归的好处是：代码简洁，实现简单； 同样的使用递归实现带来的弊端是：代码逻辑复杂，代码重入次数多，时间复杂度高
// 通俗来讲就是 “递归实现是用复杂的处理逻辑和大量的代码重入，来换取简单的实现。可以用简单的几行代码实现复杂的功能
// 计算 index = n 结果， fibonacci() 函数的重入次数为 Fibonacci[n + 1]（n > 2 时）
fn fibonacci_1(index: usize) -> usize {
    if index < 2 {
        return 1 * (1 & index);
    }
    // 无数据溢出检查的返回，输出过大会报 “栈溢出” 错误
    return fibonacci_1(index - 1) + fibonacci_1(index - 2);
    // 有数据溢出检查
    /*
    if overflow_check( fibonacci_1(index - 1), fibonacci_1(index - 2) ) {
        return fibonacci_1(index - 1) + fibonacci_1(index - 2);
    } else {
        println!("It's too big!!!");
        return 0;
    }
    */
}

fn main() {
    fibonacci_demo();

    println!("Please enter the index of the fibonacci sequence:");

    let index: usize = get_input_number();

    println!("The {index}th Fibonacci number: {}", fibonacci(index));
}
