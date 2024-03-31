use chrono::Local;
use env_logger::{fmt::Color, Builder, Env, Target};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::io::Write;

fn init_logger1() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

fn init_logger2() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    Builder::from_env(env)
        .format_level(false)
        .format_timestamp_nanos()
        .init();
}

fn init_logger3() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bg(Color::Yellow).set_bold(true);

            let timestamp = buf.timestamp();

            // writeln!(
            //     buf,
            //     "My formatted log ({}): {}",
            //     timestamp,
            //     style.value(record.args())
            // )

            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

// Specify logging filters in code instead of using an environment variable.
fn init_logger4() {
    Builder::new().filter_level(LevelFilter::max()).init();
    // Builder::new().filter_level(LevelFilter::Info).init();
}

fn init_logger5() {
    // 默认情况下，env_logger 会输出到标准错误 stderr，如果你想要输出到标准输出
    // stdout，可以使用 Builder 来改变日志对象( target ):
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();
}

struct Connection {
    port: u32,
    speed: f64,
}

fn main() {
    // init_logger1();
    // init_logger2();
    init_logger3();
    // init_logger4();
    // init_logger5();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    let conn_info = Connection {
        port: 40,
        speed: 3.20,
    };

    info!(
        "Connected to port {} at {} Mb/s",
        conn_info.port, conn_info.speed
    );
    info!(target: "connection_events", "Successful connection, port: {}, speed: {}",
      conn_info.port, conn_info.speed);
}

/*
Using `env_logger` in tests.

Log events will be captured by `cargo` and only printed if the test fails.
You can run this example by calling:

```text
cargo test --example in_tests
```

You should see the `it_does_not_work` test fail and include its log output.
 */

#[cfg(test)]
mod tests {
    use log::debug;

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[test]
    fn it_works() {
        init_logger();

        let a = 1;
        let b = 2;

        debug!("checking whether {} + {} = 3", a, b);

        assert_eq!(3, a + b);
    }

    #[test]
    fn it_does_not_work() {
        init_logger();

        let a = 1;
        let b = 2;

        debug!("checking whether {} + {} = 6", a, b);

        assert_eq!(6, a + b);
    }
}
