// 将所有出现的国际标准 ISO 8601 日期模式 YYYY-MM-DD 替换为具有斜杠的等效
// 美式英语日期模式。例如： 2013-01-15 替换为 01/15/2013
// Regex::replace_all 方法将替换整个正则表示匹配的所有内容。&str 实现了
// Replacer trait，它允许类似 $abcde 的变量引用相应的搜索匹配模式（search regex）
// 中的命名捕获组 (?P<abcde>REGEX)

use lazy_static::lazy_static;

use regex::Regex;
use std::borrow::Cow;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX: Regex =
            Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}
