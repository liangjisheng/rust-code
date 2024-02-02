// crate root(crate根)：是一个源文件，是rust编译器的起点，并构成开发者的crate的根module
// src/main.rs和src/lib.rs为Crate Root，这两个文件(任意一个)的内容形成名为crate的模块，位于整个模块树的根部。
// 其他的模块都会直接或间接地接到crate模块上，即crate模块为所有模块的祖先模块
// 绝对路径(absolute path)：从crate root开始，以crate名或者字面值crate开头。
// 相对路径(relative path)：从当前模块开始，以self(当前模块)、super(上一级模块)或当前模块的标识符开头

// rust中所有项(函数、方法、struct、enum、模块和常量)默认都是私有的
// 如果要将某些项标记为公共的，需要使用pub关键字标记它
// 父级模块无法访问子模块中的私有项(条目)：因为私有性就是为了隐藏实现细节
// 子模块里可以使用所有祖先模块中的公有和私有项(条目)

// 层级共享
// use std::io;
// use std::io::Write;
// 可以改写为
// use std::io::{self, Write};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        pub fn add_to_wait_list_1() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn task_payment() {}
    }
}

use crate::front_of_house::hosting;

// as 提供新的名称
use crate::front_of_house::hosting::add_to_wait_list as add;

// 使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的
// 如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，
// 那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：
// 我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域
pub use crate::front_of_house::hosting::add_to_wait_list_1;

fn demo1() {
    //绝对路径访问
    crate::front_of_house::hosting::add_to_wait_list();

    // 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头
    self::front_of_house::hosting::add_to_wait_list();

    front_of_house::hosting::add_to_wait_list();

    hosting::add_to_wait_list();

    add();
}

pub fn eat_at_restaurant() {
    // 绝对路径访问
    crate::front_of_house::hosting::add_to_wait_list();
    // 相对路径访问
    front_of_house::hosting::add_to_wait_list();
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

// 将结构体设置为 pub，但它的所有字段依然是私有的
// 将枚举设置为 pub，它的所有字段也将对外可见
// 原因在于，枚举和结构体的使用方式不一样。如果枚举的成员对外不可见，那该枚举将一点用都没有，
// 因此枚举成员的可见性自动跟枚举可见性保持一致，这样可以简化用户的使用。
// 而结构体的应用场景比较复杂，其中的字段也往往部分在 A 处被使用，部分在 B 处被使用，因此
// 无法确定成员的可见性，那索性就设置为全部不可见，将选择权交给程序员。

pub fn eat_at_restaurant1() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 因为seasonal_fruit字段不是公共的，所以在外部无法访问。
    // meal.seasonal_fruit = String::from("blueberries");
}

// use支持的语法
// use crate::xxx:yyy;         // 使用yyy，yyy可以是各种项
// use crate::xxx:*;           // 引入xxx下所有内容
// use crate::{xxx:yyy, zzz};  // 嵌套引入：一行里面引入多个同一棵模块子树下面的项
// use crate::xxx:zzz as z3;   // 起别名
// pub use crate::xxx:yyy;     // 重导出(re-exporting)

// pub use
// 使用use将路径导入到作用域内后，该名称在此作用域内是私有的，即当前作用域外是看不到这个use导入的路径。
// 如果要使use导入的路径在当前作用域外也能看到，可以使用pub use导入路径。
// pub use: 重导出
// 将条目引入作用域
// 该条目可以被外部代码引入到它们的作用域

fn main() {
    // eat_at_restaurant();
    eat_at_restaurant1();
}
