// 使用 reqwest::get 执行 HTTP GET 请求，然后使用 Document::from_read 将响应信息
// 解析为 HTML 文档。以“a”（锚元素）作为结构体 Name 的参数，将结构体 Name 作为条件，使
// 用 find 方法检索所有链接。在结构体 Selection 上调用 filter_map 方法，从具有
// “href” attr（属性）的链接检索所有 URL

use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/zh-CN/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
