mod class_a {
    // for use
    pub fn show_a() {
        println!("class_a::show_a()");
    }

    // for use + as
    pub mod show_info {
        pub fn show_a() {
            println!("class_a::show_info::show_a()");
        }
    }

    // for pub + use
    pub mod show {
        pub mod sh {
            pub fn ow() {
                println!("class_a::show::sh::ow()");
            }
        }
    }
    pub use self::show::sh::ow;
}

use crate::class_a::show_a;
use crate::class_a::show_info::show_a as show_b;

fn main() {
    show_a();
    show_b();
    class_a::ow();
}
