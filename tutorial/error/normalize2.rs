// 与特征对象相比，自定义错误类型麻烦归麻烦，但是它非常灵活

use std::fs::read_to_string;

fn main() -> Result<(), MyError> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, MyError> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

#[derive(Debug)]
enum MyError {
    EnvironmentVariableNotFound,
    IOError(std::io::Error),
}

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvironmentVariableNotFound
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

// 只有为自定义错误类型实现 Error 特征后，才能转换成相应的特征对象
impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
            MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
        }
    }
}

// 不得不说，真是啰嗦啊。因此在能用特征对象的时候，建议大家还是使用特征对象，
// 无论如何，代码可读性还是很重要的！
