// 程序中一般会出现两种错误：可恢复错误和不可恢复错误
// 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理
// 在 Rust 中通过 Result<T, E> 枚举类作返回值来进行异常表达

use std::fs::File;

// 返回错误
fn demof(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

fn demog(i: i32) -> Result<i32, bool> {
    let t = demof(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b),
    };
}

// Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去
// ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。所以
// ? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致
fn demog1(i: i32) -> Result<i32, bool> {
    let t = demof(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

fn main() {
    // panic!("error occured");
    println!("Hello, Rust");

    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        }
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    // if let 语法可以简化 match 语法块
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    // 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str)
    // 相当于在 Result 为 Err 时调用 panic! 宏。两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息
    // let f1 = File::open("hello.txt").unwrap();
    // let f2 = File::open("hello.txt").expect("Failed to open.");

    let r = demof(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }

    let r = demog(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }

    let r = demog1(10000);
    if let Ok(v) = r {
        println!("Ok: g1(10000) = {}", v);
    } else {
        println!("Err");
    }
}
