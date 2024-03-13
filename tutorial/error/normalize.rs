// 归一化不同的错误类型
// 在实际项目中，我们往往会为不同的错误定义不同的类型，这样做非常好，但是如果
// 你要在一个函数中返回不同的错误呢？例如：

use std::fs::read_to_string;

fn main() -> Result<(), std::io::Error> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, std::io::Error> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

// 上面的代码会报错，原因在于 render 函数中的两个 ? 返回的实际上是不同的
// 错误：env::var() 返回的是 std::env::VarError，而 read_to_string
// 返回的是 std::io::Error。

// 为了满足 render 函数的签名，我们就需要将 env::VarError 和 io::Error
// 归一化为同一种错误类型。要实现这个目的有三种方式:

// 使用特征对象 Box<dyn Error>
// 自定义错误类型
// 使用 thiserror
