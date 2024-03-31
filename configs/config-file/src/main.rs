use dotenv::dotenv;
use std::env;

mod settings;
use settings::Settings;

fn main() {
    // 读取 Cargo.toml 文件
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    println!("{} {} {}", &name, &version, &author);

    // 在访问环境变量之前检查一下，防止因读取环境变量失败导致程序恐慌。
    // 先把 dotenv 导入，然后在程序开始的地方执行 dotenv() 函数即可，这就会从当前目录或父目录中的 .env 文件中加载环境变量。
    // 如果你想指定其它路径，可以使用 crate 中提供的 from_filename 或 from_path 这两个函数。
    // 好，那么调用 dotenv() 之后为什么还要调用 ok() 方法？
    // 首先，dotenv() 返回的是 Result<PathBuf> 类型，如果返回值不使用的话，就会发出一个警告：
    // 调用 ok() 之后，会把 Result 转化为 Option，而 Option 就不会产生未使用 Result 的警告了。
    // 那么，为什么不使用 unwrap()？
    // 因为在生产环境中，你不会使用 .env 这个文件，你应该使用真实的环境变量，这时 dotenv() 函数就会加载失败，如果使用 unwrap()，那么你的程序就会停止运行。
    // 所以这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    println!("{:?}", database_url);

    let setting = Settings::get();
    println!("{:?}", setting.database.url);
    println!("{:?}", setting.log);
}
