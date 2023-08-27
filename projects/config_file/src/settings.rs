use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub debug: bool,
    pub debug_sql: bool,
    pub log_root: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub log: Log,
}

impl Default for Settings {
    fn default() -> Self {
        let file_path = "./config/config.toml";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        // 最后不能有分号, 加上就变成了语句, 不会有值返回, 去掉分号变成表达式就回返回值
        toml::from_str(&str_val).expect("Parsing the configuration file failed")
    }
}

impl Settings {
    pub fn get<'a>() -> &'a Self {
        // 给静态变量延迟赋值的宏
        lazy_static! {
            static ref CACHE: Settings = Settings::default();
        }
        &CACHE
    }
}
