// https://rust-book.junmajinlong.com/ch102/tracing.html

// tracing可以记录结构化的日志，可以按区间(span)记录日志，例如一个函数可以作为一个区间单元，
// 也可以自行指定何时进入(span.enter())区间单元
// tracing有TRACE DEBUG INFO WARN ERROR共5个日志级别，其中TRACE是最详细的级别

use chrono::Local;
use log;
use std::io;
use tracing::*;
use tracing_subscriber::fmt::{self, format::Writer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{self, fmt::time::FormatTime, util::SubscriberInitExt};

use std::net::Ipv4Addr;

use tokio::{io::AsyncWriteExt, net::TcpStream};

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
fn trace_file() {
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

struct Connection {
    port: u32,
    speed: f64,
}

fn t1() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 调用 `log` 包的 `info!`
    log::info!("Hello world");

    let foo = 42;
    // 调用 `tracing` 包的 `info!`
    tracing::info!(foo, "Hello from tracing");

    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let conn = Connection {
        port: 40,
        speed: 3.20,
    };

    info!(conn.port, "connected to {:?}", addr);
    info!(
        target: "connection_events",
        ip = ?addr,
        conn.port,
        ?conn.speed,
    );
    info!(name: "completed", "connection to {:?}", addr);

    // 除了以字段的方式记录信息，我们还可以使用格式化字符串的方式( 同 println! 、format! )
    let question = "question";
    let answer = 42;
    event!(
        Level::DEBUG,
        question.answer = answer,
        question.tricky = true,
        "the answer to {} is {}.",
        question,
        answer
    );

    // 日志输出 -> DEBUG test_tracing: the answer to question is 42. question.answer=42 question.tricky=true
}

// 相比起日志只能记录在某个时间点发生的事件，span 最大的意义就在于它可以记录一个过程，
// 也就是在某一段时间内发生的事件流。既然是记录时间段，那自然有开始和结束:

fn span1() {
    let span = span!(Level::TRACE, "my_span");

    // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
    let enter = span.enter();
    // 这里开始进入 `my_span` 的上下文
    // 下面执行一些任务，并记录一些信息到 `my_span` 中
    // ...
} // 这里 enter 将被 drop，`my_span` 也随之结束

// tracing 的 span 不仅仅是上面展示的基本用法，它们还可以进行嵌套

fn span2() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let scope = span!(Level::DEBUG, "foo");
    let _enter = scope.enter();
    info!("Hello in foo scope");
    debug!("before entering bar scope");
    {
        let scope = span!(Level::DEBUG, "bar", ans = 42);
        let _enter = scope.enter();
        debug!("enter bar scope");
        info!("In bar scope");
        debug!("end bar scope");
    }
    debug!("end bar scope");
}

// span! 和 event! 宏都需要设定相应的日志级别，而且它们支持可选的 target 或
// parent 参数( 只能二者选其一 )，该参数用于描述事件发生的位置，如果父 span
// 没有设置，target 参数也没有提供，那这个位置默认分别是当前的 span 和 当前的模块。

// Event 代表了某个时间点发生的事件，这方面它跟日志类似，但是不同的是，Event 还可以产生
// 在 span 的上下文中。

fn span3() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let s = span!(Level::TRACE, "my span");
    // 没进入 span，因此输出日志将不会带上 span 的信息
    event!(target: "app_events", Level::INFO, "something has happened 1!");

    // 进入 span ( 开始 )
    let _enter = s.enter();
    // 没有设置 target 和 parent
    // 这里的对象位置分别是当前的 span 名和模块名
    event!(Level::INFO, "something has happened 2!");
    // 设置了 target
    // 这里的对象位置分别是当前的 span 名和 target
    event!(target: "app_events",Level::INFO, "something has happened 3!");

    let span = span!(Level::TRACE, "my span 1");
    // 这里就更为复杂一些，留给大家作为思考题
    event!(parent: &span, Level::INFO, "something has happened 4!");
}

fn span4() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let user = "ferris";
    let s = span!(Level::TRACE, "login", user);
    let _enter = s.enter();

    info!(welcome = "hello", user);
    // 下面一行将报错，原因是这种写法是格式化字符串的方式，必须使用 info!("hello {}", user)
    // info!("hello", user);

    // 日志输出 -> INFO login{user="ferris"}: test_tracing: welcome="hello" user="ferris"
}

struct User {
    name: &'static str,
    email: &'static str,
}

fn event1() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let user = "ferris";
    let email = "ferris@rust-lang.org";
    event!(Level::TRACE, user, user.email = email);

    // 还可以使用结构体
    let user = User {
        name: "ferris",
        email: "ferris@rust-lang.org",
    };

    // 直接访问结构体字段，无需赋值即可使用
    span!(Level::TRACE, "login", user.name, user.email);

    // 字段名还可以使用字符串
    event!(
        Level::TRACE,
        "guid:x-request-id" = "abcdef",
        "type" = "request"
    );

    // 日志输出 ->
    // TRACE test_tracing: user="ferris" user.email="ferris@rust-lang.org"
    // TRACE test_tracing: guid:x-request-id="abcdef" type="request"
}

fn event2() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 在 span 的上下文之外记录一次 event 事件
    event!(Level::INFO, "something happened");

    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    // 在 "my_span" 的上下文中记录一次 event
    event!(Level::DEBUG, "something happened inside my_span");

    // 我们可以通过语法 field_name = field_value 来输出结构化的日志
    event!(
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything"
    );
}

// ? 符号用于说明该字段将使用 fmt::Debug 来格式化
// % 说明字段将用 fmt::Display 来格式化

#[derive(Debug)]
struct MyStruct {
    field: &'static str,
}

fn event3() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let my_struct = MyStruct {
        field: "Hello world!",
    };

    // `my_struct` 将使用 Debug 的形式输出
    event!(Level::TRACE, greeting = ?my_struct);
    // 等价于:
    event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));

    // 下面代码将报错, my_struct 没有实现 Display
    // event!(Level::TRACE, greeting = my_struct);

    // 日志输出 -> TRACE test_tracing: greeting=MyStruct { field: "Hello world!" }

    // `my_struct.field` 将使用 `fmt::Display` 的格式化形式输出
    event!(Level::TRACE, greeting = %my_struct.field);
    // 等价于:
    event!(
        Level::TRACE,
        greeting = tracing::field::display(&my_struct.field)
    );

    // 作为对比，大家可以看下 Debug 和正常的字段输出长什么样
    event!(Level::TRACE, greeting = ?my_struct.field);
    event!(Level::TRACE, greeting = my_struct.field);

    // 下面代码将报错, my_struct 没有实现 Display
    // event!(Level::TRACE, greeting = %my_struct);
}

// 如果想要将某个函数的整个函数体都设置为 span 的范围，最简单的方法就是为函数标记上
// #[instrument]，此时 tracing 会自动为函数创建一个 span，span 名跟函数名相同，
// 在输出的信息中还会自动带上函数参数。

#[instrument]
fn foo(ans: i32) {
    info!("in foo");
}

fn instr1() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    foo(42);
}

// 对于没有内置 tracing 支持或者无法使用 #instrument 的函数，例如外部库的函数，
// 我们可以使用 Span 结构体的 in_scope 方法，它可以将同步代码包裹在一个 span 中：

fn in_scope() {
    // let json = info_span!("json.parse").in_scope(|| serde_json::from_slice(&buf))?;
}

#[instrument]
async fn write(stream: &mut TcpStream) -> io::Result<usize> {
    let result = stream.write(b"hello world\n").await;
    info!("wrote to stream; success={:?}", result.is_ok());
    result
}

fn main() {
    // trace_file();
    t1();
    // span2();
    // span3();
    // span4();
    // event1();
    // event2();
    // event3();
    // instr1();
}
