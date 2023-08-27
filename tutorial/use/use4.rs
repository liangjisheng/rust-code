// mod中结构体默认为private，需要公开结构体路径，同时结构体字段默认也为private
// 如要访问结构体字段，需要将该字段设置为public

mod student {
    pub struct Stu {
        id: u32,          //id默认为private ， 不可以被外部函数访问
        pub name: String, // name 字段是public的，可以被外部函数访问
    }

    impl Stu {
        pub fn new_stu(name: &str) -> Stu {
            // 类似构造函数，name public是可以通过外部传参来修改的。id为private ，无法被直接修改
            Stu {
                id: 999,
                name: String::from(name),
            }
        }

        pub fn set_id(&mut self, id: u32) {
            // Private 字段只能被同一包内访问，同时若要修改，需要传入mut参数，否则字u32、str等是不可被修改的
            self.id = id;
        }

        pub fn get_id(&self) -> u32 {
            // 私有字段的访问方式
            self.id
        }
    }
}

pub fn change_name() {
    let mut s1 = student::Stu::new_stu("name_ori");
    //  println!("id = {}",s1.id);//error  , private id  // 私有字段无法被直接访问
    println!("name = {}", s1.name); //公有字段可以被直接访问
    s1.set_id(1000); //私有字段的修改，需要定义mut参数，只能修改mut参数，不能修改str等字段
    let id = s1.get_id();
    println!("id = {}", id);
}

fn main() {
    change_name();
}
