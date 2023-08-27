pub mod movies {
    pub fn play_movie(name: String) {
        println!("Playing movie {}", name);
    }

    pub mod english {
        pub mod comedy {
            pub fn play_comedy(name: String) {
                println!("Playing comedy movie {}", name);
            }
        }
    }
}

// use 关键字用于文件头部预先引入需要用到的外部模块中的函数或结构体
use movies::play_movie;

use movies::english::comedy::play_comedy; // 导入公开的模块

fn main() {
    movies::play_movie("Herold and Kumar".to_string());
    play_movie("Herold and Kumar ".to_string());

    // 短路径语法
    play_comedy("Herold and Kumar".to_string());
    // 全路径语法
    movies::english::comedy::play_comedy("Airplane!".to_string());
}
