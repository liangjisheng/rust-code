use crate::src_a::a_echo;

//如何引入a.rs或c.rs中的函数
pub fn b_echo() {
    println!("b_echo! => call a()!");
    a_echo();
}
