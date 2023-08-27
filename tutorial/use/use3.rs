mod class {
    mod func {
        // 通过绝对路径被调用的函数
        fn show() {
            println!("show");
        }
    }
}

mod class_a {
    // for pub + use
    pub mod show {
        pub mod sh {
            pub fn ow() {
                println!("class_a::show::sh::ow()");
            }
        }
    }

    // self 代表 class_a 模块
    pub use self::show::sh::ow;
}

fn main() {
    class_a::ow(); // 直接从mod后调用
}
