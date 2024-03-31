// 如下实例使用 form_urlencoded::byte_serialize 将字符串编码为
// application/x-www-form-urlencoded 表单语法，随后使用
// form_urlencoded::parse 对其进行解码。这两个函数都返回迭代器，
// 然后这些迭代器聚集为 String

use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
