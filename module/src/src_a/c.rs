// 注意，crate不能用self替换crate
use crate::src_a::a::*;

pub fn c_echo() {
    println!("c_echo!");
    a_echo();
}
