#[derive(Debug)]
// 导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

fn main() {
    let runoob = Site {
        domain: String::from("www.baidu.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    println!(
        "{}, {}, {}, {}",
        runoob.domain, runoob.name, runoob.nation, runoob.found
    );
    println!("struct is {:?}", runoob);
    println!("struct is {:#?}", runoob);

    let domain = String::from("www.baidu.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };
    println!(
        "{}, {}, {}, {}",
        runoob.domain, runoob.name, runoob.nation, runoob.found
    );
    println!("struct is {:?}", runoob);
    // 如果属性较多的话可以使用 {:#?} 将结构体展开
    println!("struct is {:#?}", runoob);

    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
