// 通过 Timelike 获取当前 UTC DateTime 及其时/分/秒，通过 Datelike 获取其年/月/日/工作日

use chrono::{Datelike, Timelike, Utc};

fn time_check() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}

use chrono::{NaiveDate, NaiveDateTime};

// 使用 NaiveDateTime::timestamp 将由 NaiveDate::from_ymd 生成的日期和由
// NaiveTime::from_hms 生成的时间转换为 UNIX 时间戳。然后，它使用
// NaiveDateTime::from_timestamp 计算自 UTC 时间 1970 年 01 月 01 日 00:00:00
// 开始的 10 亿秒后的日期。

fn time_transfer() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time,
        date_time.timestamp()
    );

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds
    );
}

// 使用 Utc::now 获取并显示当前 UTC 时间。使用 DateTime::to_rfc2822 将当前时间
// 格式化为熟悉的 RFC 2822 格式，使用 DateTime::to_rfc3339 将当前时间格式化为熟悉
// 的 RFC 3339 格式，也可以使用 DateTime::format 自定义时间格式。

use chrono::DateTime;

fn time_format_to_string() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );
}

// 熟悉的时间格式 RFC 2822、RFC 3339，以及自定义时间格式，通常用字符串表达。要将这些
// 字符串解析为 DateTime 结构体，可以分别用 DateTime::parse_from_rfc2822、
// DateTime::parse_from_rfc3339，以及 DateTime::parse_from_str

use chrono::format::ParseError;
use chrono::NaiveTime;

fn time_parse() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}

fn main() {
    // time_check();
    // time_transfer();
    // time_format_to_string();
    let _ = time_parse();
}
