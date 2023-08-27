// crate root(crate根)：是一个源文件，是rust编译器的起点，并构成开发者的crate的根module
// src/main.rs和src/lib.rs为Crate Root，这两个文件(任意一个)的内容形成名为crate的模块，位于整个模块树的根部。
// 其他的模块都会直接或间接地接到crate模块上，即crate模块为所有模块的祖先模块
// 绝对路径(absolute path)：从crate root开始，以crate名或者字面值crate开头。
// 相对路径(relative path)：从当前模块开始，以self(当前模块)、super(上一级模块)或当前模块的标识符开头

// rust中所有项(函数、方法、struct、enum、模块和常量)默认都是私有的
// 如果要将某些项标记为公共的，需要使用pub关键字标记它
// 父级模块无法访问子模块中的私有项(条目)：因为私有性就是为了隐藏实现细节
// 子模块里可以使用所有祖先模块中的公有和私有项(条目)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn task_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径访问
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径访问
    front_of_house::hosting::add_to_waitlist();
}

// super关键字：用来访问父级模块路径中的内容
mod restaurant {
    fn serve_order() {}

    mod back_of_house {
        fn cook_order() {}
        fn action() {
            super::serve_order();
            cook_order();
        }
    }
}

// struct中的字段默认是私有的，除非使用pub修饰字段使其变成公有的
mod back_of_house {
    // Breakfast 在 back_of_house 同级中可见
    pub struct Breakfast {
        pub toast: String,      // toast字段外部可见
        seasonal_fruit: String, // seasonal_fruit 字段外部不可见
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Appetizer 被标记为pub，即使它的变体没有被标记为pub，也全是外部可见的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant1() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 因为seasonal_fruit字段不是公共的，所以在外部无法访问。
    // meal.seasonal_fruit = String::from("blueberries");
}

fn main() {
    // eat_at_restaurant();
    eat_at_restaurant1();
}
