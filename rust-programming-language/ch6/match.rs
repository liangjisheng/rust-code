#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值
// 分支后的逗号是可选的

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn m1() {
    let c1 = Coin::Penny;
    println!("{}", value_in_cents(c1));
    let c2 = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c2));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 最后一个分支则涵盖了所有其他可能的值，模式是我们命名为 other 的一个变量
    // other 分支的代码通过将其传递给 move_player 函数来使用这个变量

    // 我们必须将通配分支放在最后，因为模式是按顺序匹配的。如果我们在通配分支后添加其他分支，
    // Rust 将会警告我们，因为此后的分支永远不会被匹配到
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，
    // 请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        6 => (),
        _ => re_roll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn re_roll() {}

fn m2() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 使用 if let 这种更短的方式编写
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // 换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值

    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count {}", count);
}

fn main() {
    // m1();
    m2();
}
