// error-chain 也是简单好用的库，可惜不再维护了，但是我觉得它依然
// 可以在合适的地方大放光彩，值得大家去了解下。
// https://github.com/rust-lang-deprecated/error-chain

use std::fs::read_to_string;

error_chain::error_chain! {
  foreign_links {
    EnvironmentVariableNotFound(::std::env::VarError);
    IOError(::std::io::Error);
  }
}

fn main() -> Result<()> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}
