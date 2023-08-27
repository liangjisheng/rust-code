// Rust 中有两种简单的访问权: 公共(public)和私有(private),默认为私有的

mod nation {
    pub mod government {
        pub fn govern() {}
    }

    mod congress {
        pub fn legislate() {}
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}

// 如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的
// 所以如果想使用模块中的结构体以及其字段，需要 pub 声明
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 枚举类枚举项可以内含字段，但不具备类似 public,private 的可见性
mod SomeModule {
    pub enum Person {
        King { name: String },
        Queen,
    }
}

fn main() {
    // 对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问
    nation::government::govern();

    eat_at_restaurant();

    let person = SomeModule::Person::King {
        name: String::from("Blue"),
    };
    match person {
        SomeModule::Person::King { name } => {
            println!("{}", name);
        }
        _ => {}
    }
}
