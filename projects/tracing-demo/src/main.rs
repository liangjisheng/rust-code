// https://rust-book.junmajinlong.com/ch102/tracing.html

// tracing可以记录结构化的日志，可以按区间(span)记录日志，例如一个函数可以作为一个区间单元，
// 也可以自行指定何时进入(span.enter())区间单元
// tracing有TRACE DEBUG INFO WARN ERROR共5个日志级别，其中TRACE是最详细的级别

use chrono::Local;
use std::io;
use tracing::*;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

const fn east8() -> Option<chrono::FixedOffset> {
    chrono::FixedOffset::east_opt(8 * 3600)
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        // 本地时区, 在不同地方运行时间格式不一样
        // write!(w, "{}", Local::now().format("%FT%T%.3f"))

        // 指定时区为东八区
        let now = chrono::Utc::now().with_timezone(&east8().unwrap());
        write!(w, "{}", now.format("%FT%T%.3f"))
    }
}

// 通过instrument属性，直接让整个函数或方法进入span区间，且适用于异步函数async fn fn_name(){}
// 参考：https://docs.rs/tracing/latest/tracing/attr.instrument.html
// #[tracing::instrument(level = "info")]
#[instrument]
fn test_trace(n: i32) {
    // #[instrument]属性表示函数整体在一个span区间内，因此函数内的每一个event信息中都会额外带有函数参数
    // 在函数中，只需发出日志即可
    event!(Level::TRACE, answer = 42, "trace2: test_trace");
    trace!(answer = 42, "trace1: test_trace");
    info!(answer = 42, "info1: test_trace");
}

// 在可执行程序中，需初始化tracing subscriber来收集、筛选并按照指定格式来记录日志
fn main() {
    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily("./log", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTimer);

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout) // 写入标准输出
        .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    // init() 方法会调用 set_global_default()，使得指定的 Subscriber 被设置为所有线程的全局
    // Subscriber，所有的日志记录都会被该全局 Subscriber 给接收

    test_trace(33);
    trace!("tracing-trace");
    debug!("tracing-debug");
    info!("tracing-info");
    warn!("tracing-warn");
    error!("tracing-error");
}
