use std::thread;
use std::time::{self, Instant};

// 测量从 time::Instant::now 开始运行的时间 time::Instant::elapsed。
fn expensive_function() {
    thread::sleep(time::Duration::from_secs(1));
}

fn time_elapsed() {
    let five_seconds = time::Duration::new(5, 0);
    let five_seconds_and_five_nanos = five_seconds + time::Duration::new(0, 5);

    assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
    assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);

    let ten_millis = time::Duration::from_millis(10);

    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!(
        "Time elapsed in expensive_function() is: {:?}",
        duration.as_secs()
    );

    let now = Instant::now();
    let max_seconds = u64::MAX / 1_000_000_000;
    let duration = time::Duration::new(max_seconds, 0);
    println!("{:?}", now + duration);
}

// 使用 DateTime::checked_add_signed 计算并显示两周之后的日期和时间，使用
// DateTime::checked_sub_signed 计算并显示前一天的日期。如果无法计算出日期
// 和时间，这些方法将返回 None

use chrono::{self, DateTime, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(chrono::Duration::days(1))
}

fn time_add() {
    let now = Utc::now();
    println!("now {}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(chrono::Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(chrono::Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("after three weeks {}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(chrono::Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}

// 使用 offset::Local::now 获取本地时间并显示，然后使用 DateTime::from_utc
// 结构体方法将其转换为 UTC 标准格式。最后，使用 offset::FixedOffset 结构体，
// 可以将 UTC 时间转换为 UTC+8 和 UTC-2

use chrono::{FixedOffset, Local};

fn time_zone() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );
}

fn main() {
    // time_elapsed();
    // time_add();
    time_zone();
}
