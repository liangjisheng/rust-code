// enum只要public，则内部都为pub，可以被直接访问

mod class {
    pub enum Sta {
        Ret { sta: u32 },
        Nothing,
    }
}

fn enum_demo() {
    let ret = class::Sta::Ret { sta: 1 }; // Ret及其字段，可以访问。
    match ret {
        class::Sta::Ret { sta } => {
            println!("get sta : {}", sta);
        }
        _ => {}
    }
}

fn main() {
    enum_demo();
}
