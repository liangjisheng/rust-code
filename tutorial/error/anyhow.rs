// anyhow 和 thiserror 是同一个作者开发的，这里是作者关于 anyhow 和 thiserror 的原话：
// https://github.com/dtolnay/anyhow

use std::fs::read_to_string;

use anyhow::Result;

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
