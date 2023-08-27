struct User {
    username: String,
    age: i32,
}

// 方法与函数类似，使用 fn 关键字声明，拥有参数和返回值，不同的是方法在结构体的上下文中定义，
// 方法的第一个参数始终为 self 表示调用该方法的结构体实例

impl User {
    fn info(self) {
        print!("我是: {}, 永远 {}\n", self.username, self.age);
    }
}

fn struct1() {
    let user1 = User {
        username: String::from("ljs"),
        age: 18,
    };
    print!("我是: {}, 永远 {}\n", user1.username, user1.age); // 我是: ljs, 永远 18
    user1.info();
}

// Rust 提供了 Debug 模式的调试打印方法, # 表示引入调试(Debug)方法
#[derive(Debug)]
struct Student {
    name: String,
    class: String,
    score: u64,
    pass: bool,
}

// 定义一个方法, 方法中的第一个参数是 self，表示当前的结构对象
impl Student {
    // impl 定义的结构块还有另一个重要的功能，其中定义的函数允许不以 self 作为第一个参数，
    // 而使用其它参数或者没有参数，这类函数则被称为关联函数 (Associated Functions)
    // 需要注意的是这样的定义则被称为函数不是方法了，因为它并不只作用于一个结构体对象
    fn new(name: String, class: String, score: u64, pass: bool) -> Student {
        Student {
            name,
            class,
            score,
            pass,
        }
    }

    fn pass(&self) -> bool {
        if self.score >= 60 {
            true
        } else {
            false
        }
    }
}

fn struct2() {
    let mut stu1 = Student {
        name: String::from("stu1"),
        class: String::from("10-01"),
        pass: true,
        score: 89,
    };

    stu1.class = String::from("10-03");
    println!("Student Name: {} Class: {}", stu1.name, stu1.class);
    // :? 这个符号告诉 println! 将使用 Debug 的输出格式
    println!("Student: {:?}", stu1);
    // 另一个符号也表示 Debug 模式输出 :#?
    println!("Student: {:#?}", stu1);
    if stu1.pass() {
        println!("{} is pass.", stu1.name);
    } else {
        println!("{} is no pass.", stu1.name);
    }

    let mut stu2 = build_student(String::from("stu2"), String::from("10-03"), 86);
    println!("Student Name: {} Class: {}", stu2.name, stu2.class);
    println!("Student: {:?}", stu2);

    // 在构造结构体时，经常会遇到这种情况，创建的结构体的大部分信息与之前的相同，
    // 将会直接用之前结构体中的实例计算出的结果来初始化新的结构体
    let mut stu3 = Student {
        name: String::from("stu3"),
        class: stu2.class,
        score: stu2.score,
        pass: stu2.pass,
    };
    println!("Student Name: {} Class: {}", stu3.name, stu3.class);
    println!("Student: {:?}", stu3);

    // .. 语法表示结构体中剩余的成员全部用 .. 后的实例中的值初始化
    let mut stu4 = Student {
        name: String::from("stu4"),
        ..stu3
    };
    println!("Student Name: {} Class: {}", stu4.name, stu4.class);
    println!("Student: {:?}", stu4);

    // 使用 :: 来调用关联函数
    let mut stu5 = Student::new(String::from("stu5"), String::from("class1"), 89, true);
    println!("Student: {:?}", stu5);
}

fn build_student(name: String, class: String, score: u64) -> Student {
    // 可以将一个函数体的最后部分构造一个结构体的实例，可以隐式的返回该实例。
    // 若函数中的参数名与结构体成员名相同则可以简化写法，无需重复的写 name: name 这样，
    // 而是可以简单的直接写 name 即可
    Student {
        name,
        class,
        score,
        pass: {
            if score >= 60 {
                true
            } else {
                false
            }
        },
    }
}

// 结构体支持定义为元组类型的结构体，又名元组结构体（Tuple Structs）
// 其结合了元组类型的特点，即结构体中的各字段没有具体的名字，只有类型
struct Human(u32, u32, u32);
struct Car(u32, u32, u32);
struct Tree(u32, u32, u32);

fn struct3() {
    let bizhe = Human(123, 176, 61);
    let porsche_tancan = Car(321, 2100, 86700);
    let wu_tong = Tree(12, 38, 430);

    println!("BiZhe: {} age, {} cm, {} kg", bizhe.0, bizhe.1, bizhe.2);
    println!(
        "Porsche_Tancan: {} hp, {} kg, {} $",
        porsche_tancan.0, porsche_tancan.1, porsche_tancan.2
    );
    println!(
        "WuTong: {} age, {} cm, {} cm",
        wu_tong.0, wu_tong.1, wu_tong.2
    );
}

fn main() {
    // struct1();
    // struct2();
    // struct3();
}
