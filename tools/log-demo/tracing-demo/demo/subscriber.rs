// 有时候有需求区分不同的日志，或者不同区域使用不同格式的日志

use tracing::*;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    // 先创建一个SubScriber，准备作为默认的全局SubScriber
    // finish()将完成SubScriber的构建，返回一个SubScriber
    let default_logger = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .finish();

    // 这段代码不记录任何日志，因为还未开启全局SubScriber
    {
        info!("nothing will log");
    }

    // 从此开始，将default_logger设置为全局SubScriber
    default_logger.init();

    // 创建一个无颜色显示的且只记录ERROR级别的SubScriber
    let tmp_logger = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(Level::ERROR)
        .with_line_number(true)
        .finish();

    // 使用with_default，可将某段代码使用指定的SubScriber而非全局的SubScriber进行日志记录
    tracing::subscriber::with_default(tmp_logger, || {
        error!("log with tmp_logger, only log ERROR logs");
    });

    // NoSubscriber 是不记录任何日志记录的Subscriber，它会丢弃所有数据
    let sc = tracing::subscriber::NoSubscriber::default();
    tracing::subscriber::with_default(sc, || {
        // 不会被记录
        tracing::info!("This will be logged to stdout");
    });

    info!("log with Global logger");
}
