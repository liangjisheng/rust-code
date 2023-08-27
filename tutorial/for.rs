// loop 表达式会无限的循环执行代码块，如果想终止循环，可配合 break 语句使用

// 从循环中获得返回值，我们可以利用这一点来获取循环退出的原因，
// 从而可以分析循环操作是否完成或成功，当然还有其它的用途
fn loop1() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result); // 20
}

// 利用循环标签消除多个循环之间的歧义
// 当为循环指定了一个循环标签后，那么则可以使用 break 和 continue 指定作用于哪个（层） loop 循环
// 循环标签的开头必须以单引号开头
fn loop2() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while1() {
    // while
    let mut counter = 3;
    while counter != 0 {
        println!("counter: {}", counter);
        counter -= 1;
    }
    println!("end");
}

fn for1() {
    let arr = ['a', 'b', 'c'];
    for element in arr.iter() {
        println!("element: {}", element);
    }
    println!("end");

    for element in arr {
        println!("the value is: {element}");
    }
    println!("end");

    for number in 1..4 {
        println!("number：{}", number);
    }
    //反转范围
    for number in (1..4).rev() {
        println!("number：{}", number);
    }
    println!("end");
}

fn for2() {
    for i in 1..11 {
        if i == 5 {
            continue;
        }
        print!("{} ", i);
    }
}

fn main() {
    // loop1();
    // loop2();
    // while1();
    // for1();
    for2();
}
