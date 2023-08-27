use std::fs::File;

// unwrap() 函数返回操作成功的实际结果。如果操作失败，它会调用 panic!() 并输出默认的错误消息
// 函数 expect() 当 self 是 Ok 或 Some 则返回包含的值。否则调用panic!() 输出自定义的错误并退出程序

fn r1() {
    // main.jpg 文件不存在
    let f = File::open("main.jpg");
    // 捕捉错误信息并恢复程序运行
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        }
        Err(e) => {
            println!("file not found \n{:?}", e); // 处理错误
        }
    }

    // 不存在就会 panic
    // let f = File::open("main.jpg").expect("File not able to open");

    println!("end of main");
}

fn is_even(no: i32) -> Result<bool, String> {
    return if no % 2 == 0 {
        Ok(true)
    } else {
        Err("NOT_AN_EVEN".to_string())
    };
}

fn r2() {
    let result = is_even(13);
    match result {
        Ok(d) => {
            println!("no is even {}", d);
        }
        Err(msg) => {
            println!("Error msg is {}", msg);
        }
    }

    let result = is_even(10).unwrap();
    println!("result is {}", result);
}

fn main() {
    r1();
    // r2();
}
