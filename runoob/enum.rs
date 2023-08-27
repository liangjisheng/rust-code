#[derive(Debug)]
// 导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体

enum Book {
    Papery,
    Electronic,
}

// 可以为枚举类成员添加元组属性描述
enum Book1 {
    Papery(u32),
    Electronic(String),
}

// 为属性命名, 请注意并不能像访问结构体字段一样访问枚举类绑定的属性 访问的方法在 match 语法中
enum Book2 {
    Papery { index: u32 },
    Electronic { url: String },
}

fn main() {
    let book = Book::Papery;
    println!("{:?}", book);

    let _book1 = Book1::Papery(1001);
    let _ebook1 = Book1::Electronic(String::from("url://..."));
    // println!("{:?}", book1);
    // println!("{:?}", ebook1);

    let book2 = Book2::Papery { index: 1001 };
    let ebook2 = Book2::Electronic {
        url: String::from("url..."),
    };
    // println!("{:?}", book2);
    // println!("{:?}", ebook2);

    match book2 {
        Book2::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book2::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}
