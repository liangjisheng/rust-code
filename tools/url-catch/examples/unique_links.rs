// 使用 reqwest::get 获取 MediaWiki 页面的源代码，然后使用 Regex::captures_iter
// 查找内部和外部链接的所有条目。使用智能指针 Cow 可以提供对借用数据的不可变引用，避免分
// 配过多的字符串

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;

fn extract_links(content: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
        )
        .unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    links
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )
    .await?
    .text()
    .await?;

    println!("{:#?}", extract_links(content.as_str()));

    Ok(())
}
