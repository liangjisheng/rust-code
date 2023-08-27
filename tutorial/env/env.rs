use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // 从环境变量 `CONFIG` 读取配置路径 `config_path`
    // 如果 `CONFIG` 未设置，采用默认配置路径
    // let config_path = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());
    let config_path = env::var("CONFIG").unwrap_or("./config.toml".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    println!("get args: {:?}", args);

    // 获取全部环境变量
    // for (k, v) in env::vars() {
    //     println!("{} => {}", k, v);
    // }

    let key = "CONFIG";
    match env::var(key) {
        Ok(val) => {
            println!("CONFIG: {}", val);
        }
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }

    Ok(())
}
