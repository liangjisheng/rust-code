// use支持的语法
// use crate::xxx:yyy;         // 使用yyy，yyy可以是各种项
// use crate::xxx:*;           // 引入xxx下所有内容
// use crate::{xxx:yyy, zzz};  // 嵌套引入：一行里面引入多个同一棵模块子树下面的项
// use crate::xxx:zzz as z3;   // 起别名
// pub use crate::xxx:yyy;     // 重导出(re-exporting)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn get_waitlist() {}
        fn some_function() {}
    }
}

mod restaurant {
    // use crate::front_of_house::hosting;
    use front_of_house::hosting;
    // crate::front_of_house::hosting::add_to_waitlist();

    use front_of_house::hosing::get_waitlist as gw;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        // hosting::some_function();    // 访问不到

        gw();
    }
}

// pub use
// 使用use将路径导入到作用域内后，该名称在此作用域内是私有的，即当前作用域外是看不到这个use导入的路径。
// 如果要使use导入的路径在当前作用域外也能看到，可以使用pub use导入路径。
// pub use: 重导出
// 将条目引入作用域
// 该条目可以被外部代码引入到它们的作用域
