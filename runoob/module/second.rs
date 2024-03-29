// 在 rust 中每一个文件就是一个 module

pub struct ClassName {
    field: i32,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName { field: value }
    }

    pub fn public_method(&self) {
        println!("from public method");
        self.private_method();
    }

    fn private_method(&self) {
        println!("from private method");
    }
}
