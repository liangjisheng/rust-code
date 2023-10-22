// trait Error: Debug + Display {
//     // 提供默认实现
//     fn source(&self) -> Option<&(dyn Error + 'static)>;
//     fn backtrace(&self) -> Option<&Backtrace>;
//     fn description(&self) -> &str;
//     fn cause(&self) -> Option<&dyn Error>;
// }

// 在 Rust 中，错误（error）是被返回（return）的，而不是被抛出（throw）的

mod m1 {
    use std::error;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    struct DivByZero;

    impl fmt::Display for DivByZero {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "division by zero error")
        }
    }

    impl error::Error for DivByZero {}

    fn safe_div(numerator: i32, denominator: i32) -> Result<i32, DivByZero> {
        if denominator == 0 {
            return Err(DivByZero);
        }
        Ok(numerator / denominator)
    }

    pub fn test_safe_div() {
        assert_eq!(safe_div(8, 2), Ok(4));
        assert_eq!(safe_div(5, 0), Err(DivByZero));
    }
}

// 传递错误的最常用方式是使用?操作符
mod m2 {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;

    fn read_file_to_string(path: &Path) -> Result<String, io::Error> {
        let mut file = File::open(path)?; // ⬆️ io::Error
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error
        Ok(contents)
    }
}

// 假定我们当前正在读取的文件内容是一串数字，并且我们想要把这些数字求和，我们可能会把函数更新成这样
mod m3 {
    use std::error;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::num::ParseIntError;
    use std::path::Path;

    /*fn sum_file(path: &Path) -> Result<i32 /*这里放置什么? */> {
        let mut file = File::open(path)?; // ⬆️ io::Error
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>()?; // ⬆️ ParseIntError
        }
        Ok(sum)
    }*/

    // 但是，现在我们的Result里的错误类型应该是什么？它要么返回一个io::Error，
    // 要么返回一个ParseIntError。我们尝试寻找第三种方式来解决这个问题，
    // 以最快最乱的方式开始，以最健壮的方式结束

    // 第一种方式就是，识别出所有实现了Error和Display的类型，这样我们把所有的错误映射 (map)
    // 到String类型并把String作为我们的错误类型:
    fn sum_file1(path: &Path) -> Result<i32, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>().map_err(|e| e.to_string())?; // ⬆️ ParseIntError -> String
        }
        Ok(sum)
    }

    // 但是，这种方式的缺点在于，我们会丢弃所有的错误类型信息，从而导致调用者在处理错误时十分困难
    // 另外一个不太明显的优点则是，我们可以定制字符串来提供更多的特定上下文信息。例如，ParseIntError
    // 通常会变成字符串“invalid digit found in string”，这个信息就非常模糊并且没有提及无效的字符串
    // 是什么或者它正在尝试解析到哪一类整数类型。如果我们正在调试这个问题，这个错误信息几乎没什么用

    // 第二种方式则是充分利用标准库中的 generic blanket impl：
    // impl<E: error::Error> From<E> for Box<dyn error::Error>;

    // 这意味着，任意的Error类型都可以通过?被隐式地转换为Box<dyn error::Error>，
    // 因此我们可以把任何可能产生错误的函数返回的Result中的错误类型设置为Box<dyn error::Error>，
    // 这样?操作符就可以帮我们完成剩下的工作：
    fn sum_file2(path: &Path) -> Result<i32, Box<dyn error::Error>> {
        let mut file = File::open(path)?; // ⬆️ io::Error -> Box<dyn error::Error>
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error -> Box<dyn error::Error>
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> Box<dyn error::Error>
        }
        Ok(sum)
    }

    // 虽然更为简洁，但是它似乎也存在着前面一种方式的缺点，即丢掉了类型信息。大多数情况下的确如此，
    // 但是如果调用者知道函数的实现细节，它们仍然可以通过使用 error::Error 上的 downcast_ref()
    // 方法来处理不同的错误类型，这与它在dyn Any类型上的作用相同
    fn handle_sum_file_errors(path: &Path) {
        match sum_file2(path) {
            Ok(sum) => println!("the sum is {}", sum),
            Err(err) => {
                if let Some(e) = err.downcast_ref::<io::Error>() {
                    // 处理 io::Error
                } else if let Some(e) = err.downcast_ref::<ParseIntError>() {
                    // 处理 ParseIntError
                } else {
                    // 我们知道 sum_file 只会返回上面错误中的其中一个
                    // 所以不会到达这个分支
                    unreachable!();
                }
            }
        }
    }
}

// 第三种方法是最稳健和类型安全的方法，它可以汇总这些不同的错误，使用一个枚举类型构建我们自己的自定义错误类型
mod m4 {
    use std::error;
    use std::fmt;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::num::ParseIntError;
    use std::path::Path;

    #[derive(Debug)]
    enum SumFileError {
        Io(io::Error),
        Parse(ParseIntError),
    }

    impl From<io::Error> for SumFileError {
        fn from(err: io::Error) -> Self {
            SumFileError::Io(err)
        }
    }

    impl From<ParseIntError> for SumFileError {
        fn from(err: ParseIntError) -> Self {
            SumFileError::Parse(err)
        }
    }

    impl fmt::Display for SumFileError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SumFileError::Io(err) => write!(f, "sum file error: {}", err),
                SumFileError::Parse(err) => write!(f, "sum file error: {}", err),
            }
        }
    }

    impl error::Error for SumFileError {
        // 这个方法的默认实现总是返回 None
        // 但是我们现在重写它，让它更有用
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            Some(match self {
                SumFileError::Io(err) => err,
                SumFileError::Parse(err) => err,
            })
        }
    }

    fn sum_file(path: &Path) -> Result<i32, SumFileError> {
        let mut file = File::open(path)?; // ⬆️ io::Error -> SumFileError
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error -> SumFileError
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> SumFileError
        }
        Ok(sum)
    }

    fn handle_sum_file_errors(path: &Path) {
        match sum_file(path) {
            Ok(sum) => println!("the sum is {}", sum),
            Err(SumFileError::Io(err)) => {
                // 处理 io::Error
            }
            Err(SumFileError::Parse(err)) => {
                // 处理 ParseIntError
            }
        }
    }
}

fn main() {
    m1::test_safe_div();
}
