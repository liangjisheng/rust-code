// Rng(random number generator)
use rand::Rng; // Rng 是一个 trait, 并且该特征必须被引入在我们使用这些函数的范围内
use std::cmp::Ordering;
use std::io;

// Result 的变体是 Ok 和 Err。Ok 代表操作执行成功，反之 Err 则表示操作失败，并包含操作失败的信息，供调试使用
// 而 io::Result 的实例可以调用她自己的 expect 函数。如果 io::Result 返回的是 Err，expect
// 则将导致程序崩溃的信息作为参数传递给 expect
// 若不使用 ‘expect’ 程序可以正常通过编译，但编译器会发出警告

fn main() {
    println!("Guess the number!");
    // rand::thread_rng 来提供将要使用的特定随机数生成器的函数：一个位于当前执行线程的本地并由操作系统确定种子的函数
    // 也可以传入参数 1..=100
    let secret_number_int = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}.", secret_number_int);
    let secret_number = secret_number_int.to_string();

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // read_line 会将标准输入的任何内容追加到字符串后（不会覆盖原有内容）
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // 在之后按完回车后。事实上 read_line 会将换行符也纳入其中，也就是说 read_line 看到的字符串是这样的 28\n
        // println!("You guessed: {}", guess);

        // trim 消除开头和结尾的所有空格或回车
        // parse 函数会将字符串解析为某种数字类型
        // 使用 expect 方法以同样的方式处理 Result。如果由于无法从字符串中创建数字而 parse 返回一个 Err 变体，
        // 则该调用( expect )将使游戏崩溃并打印我们给它的错误信息。如果可以成功地将字符串转换为数字，
        // 它将返回的变体 Ok, 并将 expect 从值中返回我们想要的数字
        // let guess: i32 = guess.trim().parse().expect("Please type a number!");

        // 处理无效输入
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ 类似一个通配符，代表着传过来的所有信息
            Err(_) => {
                println!("invalid input, input a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number_int) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yeah! You win!");
                break;
            }
        }
    }
}
